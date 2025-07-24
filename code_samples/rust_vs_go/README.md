<img src="./octogono.jpeg" height=400>

---

<img src="../../rusting-crab-logo.png" height=300>

---

[**Cleuton Sampaio**](https://linkedin.com/in/cleutonsampaio)

[**Veja no GitHub**](https://github.com/cleuton/rustingcrab/tree/main/code_samples/rust_vs_go)


# Rust vs Go

Muita gente está descobrindo **Golang** como alternativa de alto desempenho para aplicações web e APIs **REST**. E realmente é! Vejo muitos posts e relatos de pessoas convertendo apps de **NodeJS** e **Python** para **Go**. Mas... E **Rust**? Não seria também uma alternativa de alta performance e baixo **footprint**? É o que vamos ver!

> **Footprint** em software refere-se ao consumo de recursos do sistema, como memória RAM, CPU e espaço em disco, que uma aplicação utiliza durante sua execução. Em ambientes de nuvem, isso é crucial porque os custos são tipicamente baseados no consumo desses recursos. Um menor footprint significa menor uso de memória, processamento e armazenamento, o que se traduz diretamente em menores custos operacionais. Além disso, aplicações com menor footprint tendem a ser mais rápidas, escaláveis e eficientes, permitindo que mais serviços rodem na mesma infraestrutura e respondam melhor sob carga. Isso é especialmente importante em nuvem onde se paga por recurso consumido e a eficiência impacta diretamente no orçamento e performance da solução.

Este teste de performance é um comparativo que usei para convencer um cliente a adotar **Rust** como parte de sua solução. Na verdade, este é um teste de geração de IDs com **Sony Snowflake**, implementado em **Golang** (usando `FastHTTP`) e em **Rust** (usando `Axum` e `Tokyo`).

Os códigos-fonte estão na [**pasta `servers`**](./performance-test/servers/).

Usei o [**K6**](https://k6.io/) para executar os testes e criei scripts em `shell` e `python` para coletar os resultados e métricas. 

Os parâmetros de teste que usei foram: 

- Ramp Up: 30 segundos até 50 VUs.
- Teste de 1 minuto.
- Ramp Down: 30 segundos até 0. 
- Thinking time de 0.05s.

Só roda em Linux, por enquanto!

## Para rodar os testes: 

1) Instale as dependências **Python** (aconselho a criar um ambiente virtual com `venv`):

```
pip install -r requirements.txt
```
2) Instale o k6: https://k6.io/docs/getting-started/installation/

3) Dê permissão para os scripts

```
chmod +x run_tests.sh
chmod +x monitoring/resource_monitor.sh
```

## Executar testes

```
./run_tests.sh
```

## Analisar resultados

```
python3 analysis/analyze_results.py
```

## Resultado após várias execuções:

```shell
Análise para go_fasthttp:
==================================================
Verificando arquivo: results/go_fasthttp_k6_results.csv
Colunas encontradas no CSV: ['metric_name', 'timestamp', 'metric_value', 'check', 'error', 'error_code', 'expected_response', 'group', 'method', 'name', 'proto', 'scenario', 'service', 'status', 'subproto', 'tls_version', 'url', 'extra_tags', 'metadata']
Primeiras linhas do CSV:
                metric_name   timestamp  metric_value check  error  ...  subproto tls_version                           url extra_tags metadata
0                 http_reqs  1753303219      1.000000   NaN    NaN  ...       NaN         NaN  http://localhost:8888/nextid        NaN      NaN
1         http_req_duration  1753303219      0.256629   NaN    NaN  ...       NaN         NaN  http://localhost:8888/nextid        NaN      NaN
2          http_req_blocked  1753303219      0.252951   NaN    NaN  ...       NaN         NaN  http://localhost:8888/nextid        NaN      NaN
3       http_req_connecting  1753303219      0.167542   NaN    NaN  ...       NaN         NaN  http://localhost:8888/nextid        NaN      NaN
4  http_req_tls_handshaking  1753303219      0.000000   NaN    NaN  ...       NaN         NaN  http://localhost:8888/nextid        NaN      NaN

[5 rows x 19 columns]
Tempo de Resposta (ms):
   Min: 123.93
   Max: 986.48
   Média: 371.37
   Desvio: 133.66
   Total Requests: 67
   Success Rate: 100.00%
Verificando arquivo de monitoramento: results/go_fasthttp_monitoring.csv

Uso de CPU (%):
   Min: 1.60
   Max: 20.90
   Média: 4.40
   Desvio: 2.58

Uso de RAM (MB):
   Min: 6054.00
   Max: 6130.00
   Média: 6077.74
   Desvio: 16.32
Verificando arquivo de tamanho: results/go_fasthttp_binary_size.txt

Tamanho do Executável: 7,638,749 bytes (7.28 MB)

Análise para rust_axum:
==================================================
Verificando arquivo: results/rust_axum_k6_results.csv
Colunas encontradas no CSV: ['metric_name', 'timestamp', 'metric_value', 'check', 'error', 'error_code', 'expected_response', 'group', 'method', 'name', 'proto', 'scenario', 'service', 'status', 'subproto', 'tls_version', 'url', 'extra_tags', 'metadata']
Primeiras linhas do CSV:
                metric_name   timestamp  metric_value check  error  ...  subproto tls_version                           url extra_tags metadata
0                 http_reqs  1753303357      1.000000   NaN    NaN  ...       NaN         NaN  http://localhost:8888/nextid        NaN      NaN
1         http_req_duration  1753303357      0.300199   NaN    NaN  ...       NaN         NaN  http://localhost:8888/nextid        NaN      NaN
2          http_req_blocked  1753303357      0.311010   NaN    NaN  ...       NaN         NaN  http://localhost:8888/nextid        NaN      NaN
3       http_req_connecting  1753303357      0.193270   NaN    NaN  ...       NaN         NaN  http://localhost:8888/nextid        NaN      NaN
4  http_req_tls_handshaking  1753303357      0.000000   NaN    NaN  ...       NaN         NaN  http://localhost:8888/nextid        NaN      NaN

[5 rows x 19 columns]
Tempo de Resposta (ms):
   Min: 150.79
   Max: 573.50
   Média: 355.31
   Desvio: 97.15
   Total Requests: 67
   Success Rate: 100.00%
Verificando arquivo de monitoramento: results/rust_axum_monitoring.csv

Uso de CPU (%):
   Min: 1.70
   Max: 7.90
   Média: 3.71
   Desvio: 1.40

Uso de RAM (MB):
   Min: 6040.00
   Max: 6233.00
   Média: 6064.34
   Desvio: 33.67
Verificando arquivo de tamanho: results/rust_axum_binary_size.txt

Tamanho do Executável: 870,976 bytes (0.83 MB)

Análise completa! Resultados salvos em results/comparison.json
```

## Análise Completa de Performance (com desvio padrão)

### Tempo de Resposta
**Vencedor: Rust Axum**
- **Go FastHTTP**: Média 371.37ms (σ=133.66) | **Rust Axum**: Média 355.31ms (σ=97.15)
- Rust é ~4.3% mais rápido em média
- **Desvio padrão menor**: Rust é mais consistente (97.15 vs 133.66)

### Uso de CPU
**Vencedor: Rust Axum**
- **Go FastHTTP**: Média 4.40% (σ=2.58) | **Rust Axum**: Média 3.71% (σ=1.40)
- Rust usa ~15.7% menos CPU em média
- **Maior consistência**: Rust tem desvio padrão 45% menor

### Uso de Memória
**Empate técnico**
- **Go FastHTTP**: Média 6077.74 MB (σ=16.32) | **Rust Axum**: Média 6064.34 MB (σ=33.67)
- Diferença de ~0.22% (praticamente igual)
- **Go mais consistente**: Desvio padrão 51% menor

### Tamanho do Executável
**Vencedor: Rust Axum** 
- **Go FastHTTP**: 7.64 MB | **Rust Axum**: 0.87 MB
- Rust é **8.8x menor** (!)

### Conclusão Final

**Rust Axum vence em 3 das 4 categorias principais:**
1. **Tempo de resposta** (mais rápido e mais consistente)
2. **Uso de CPU** (menor consumo e maior consistência)  
3. **Tamanho do binário** (8.8x menor)

**Go FastHTTP vence apenas em:**
1. **Consistência de memória** (desvio padrão menor)

### Recomendação Final
**Escolha Rust Axum** para:
- Menor latência e maior previsibilidade
- Menor consumo de CPU
- Binário 8.8x menor (ótimo para deployment)
- Performance mais consistente

**Escolha Go FastHTTP** apenas se:
- A absoluta consistência de uso de memória for crítica
- Você preferir a simplicidade do ecossistema Go

Para um serviço como este, **Rust Axum é claramente superior** em quase todos os aspectos.



