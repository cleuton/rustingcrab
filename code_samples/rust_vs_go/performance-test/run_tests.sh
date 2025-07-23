#!/bin/bash

set -e

RESULTS_DIR="results"
mkdir -p $RESULTS_DIR

# Função para executar teste
run_test() {
    local SERVER_NAME=$1
    local BUILD_CMD=$2
    
    echo "Iniciando teste para $SERVER_NAME"
    
    # Construir o servidor
    echo "Construindo $SERVER_NAME..."
    cd servers/$SERVER_NAME
    eval $BUILD_CMD
    cd ../..
    
    # Verificar se o executável foi criado
    if [ "$SERVER_NAME" == "rust_axum" ]; then
        EXECUTABLE="./servers/$SERVER_NAME/target/release/zaptidgen-rust"
    else
        EXECUTABLE="./servers/$SERVER_NAME/zaptidgen"
    fi
    
    if [ ! -f "$EXECUTABLE" ]; then
        echo "ERRO: Executável não encontrado: $EXECUTABLE"
        return 1
    fi
    
    # Iniciar servidor em background
    echo "Iniciando servidor..."
    $EXECUTABLE > /dev/null 2>&1 &
    SERVER_PID=$!
    
    # Esperar servidor iniciar
    sleep 3
    
    # Testar se o servidor está respondendo
    if ! curl -s --max-time 5 http://localhost:8888/nextid > /dev/null; then
        echo "ERRO: Servidor não está respondendo"
        kill $SERVER_PID 2>/dev/null || true
        return 1
    fi
    
    # Iniciar monitoramento de recursos (do diretório raiz)
    echo "Iniciando monitoramento de recursos..."
    ./monitoring/resource_monitor.sh "$SERVER_NAME" 120 > /dev/null 2>&1 &
    MONITOR_PID=$!
    
    # Executar teste K6 com limite de tempo
    echo "Executando teste K6..."
    timeout 130 k6 run --out csv=$RESULTS_DIR/${SERVER_NAME}_k6_results.csv k6_tests/load_test.js || true
    
    # Parar servidor e monitor
    echo "Parando servidor e monitor..."
    kill $SERVER_PID 2>/dev/null || true
    kill $MONITOR_PID 2>/dev/null || true
    
    # Esperar processos terminarem
    sleep 2
    
    # Salvar tamanho do executável
    BINARY_SIZE=$(stat -f%z $EXECUTABLE 2>/dev/null || stat -c%s $EXECUTABLE)
    echo "$BINARY_SIZE" > $RESULTS_DIR/${SERVER_NAME}_binary_size.txt
    
    echo "Teste concluído para $SERVER_NAME"
    echo "----------------------------------------"
}

# Compilar e executar testes
echo "Iniciando bateria de testes..."

run_test "go_fasthttp" "go build -o zaptidgen ."
run_test "rust_axum" "cargo build --release"

echo "Todos os testes concluídos!"