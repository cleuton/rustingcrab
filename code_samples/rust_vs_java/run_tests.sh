#!/bin/bash
set -euo pipefail

RESULTS_DIR="results"
mkdir -p "$RESULTS_DIR"

run_test() {
  local SERVER_NAME="$1"      # ex: rust_axum | java_undertow
  local BUILD_CMD="$2"        # comando para construir
  local START_CMD="$3"        # comando para iniciar o servidor
  local ARTIFACT_PATH="$4"    # caminho do binário/jar para checagem e size

  echo "Iniciando teste para $SERVER_NAME"
  echo "Construindo $SERVER_NAME..."

  pushd "servers/$SERVER_NAME" > /dev/null
  # tenta construir; se falhar, aborta aqui
  bash -c "$BUILD_CMD"
  popd > /dev/null

  # checar artefato (binário do Rust ou JAR do Java)
  if [ ! -f "$ARTIFACT_PATH" ]; then
    echo "ERRO: Artefato não encontrado: $ARTIFACT_PATH"
    exit 1
  fi

  echo "Iniciando servidor..."
  # inicia servidor em background a partir do diretório raiz do projeto
  bash -c "$START_CMD" > /dev/null 2>&1 &
  SERVER_PID=$!

  # aguardar subir
  sleep 3

  # health-check simples
  if ! curl -s --max-time 5 http://localhost:8888/nextid > /dev/null; then
      echo "ERRO: Servidor não está respondendo"
      kill $SERVER_PID 2>/dev/null || true
      exit 1
  fi

  echo "Iniciando monitoramento de recursos..."
  ./monitoring/resource_monitor.sh "$SERVER_NAME" 120 > /dev/null 2>&1 &
  MONITOR_PID=$!

  echo "Executando teste K6..."
  timeout 130 k6 run --out csv=$RESULTS_DIR/${SERVER_NAME}_k6_results.csv k6_tests/load_test.js || true

  echo "Parando servidor e monitor..."
  kill $SERVER_PID 2>/dev/null || true
  kill $MONITOR_PID 2>/dev/null || true
  sleep 2

  # tamanho do artefato
  BINARY_SIZE=$(stat -f%z "$ARTIFACT_PATH" 2>/dev/null || stat -c%s "$ARTIFACT_PATH")
  echo "$BINARY_SIZE" > "$RESULTS_DIR/${SERVER_NAME}_binary_size.txt"

  echo "Teste concluído para $SERVER_NAME"
  echo "----------------------------------------"
}

echo "Iniciando bateria de testes..."

# --- RUST ---
run_test \
  "rust_axum" \
  "cargo build --release" \
  "./servers/rust_axum/target/release/zaptidgen-rust" \
  "./servers/rust_axum/target/release/zaptidgen-rust"

# --- JAVA ---
# Build Java: tenta ./gradlew e, se falhar, cai para gradle do sistema.
# Isso resolve wrapper velho ou ausente.
JAVA_BUILD_CMD='( ./gradlew -q --no-daemon shadowJar || gradle -q shadowJar )'

run_test \
  "java_undertow" \
  "mvn -q -DskipTests package" \
  "java -jar servers/java_undertow/target/java_undertow-1.0.0-all.jar" \
  "./servers/java_undertow/target/java_undertow-1.0.0-all.jar"

echo "Todos os testes concluídos!"
