<img src="./rust_vs_java.png" height=400>

---

<img src="../../rusting-crab-logo.png" height=300>

---

[**Cleuton Sampaio**](https://linkedin.com/in/cleutonsampaio)

[**Veja no GitHub**](https://github.com/cleuton/rustingcrab/tree/main/code_samples/rust_vs_java)


# Rust vs Java

Era inevitável... O combate do século aconteceu e você verá com exclusividade (e sem pagar Pay-per-view) aqui, no **RustingCrab**.

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

# Benchmark Report — Rust (axum) vs Java (Undertow)

## 1. Desempenho (latência e throughput)

| Métrica              | Rust (axum) | Java (Undertow) |
|----------------------|-------------|-----------------|
| Latência média       | 0,38 ms     | 0,45 ms         |
| Desvio padrão        | 0,21 ms     | 0,26 ms         |
| Máximo observado     | 5,34 ms     | 3,65 ms         |
| Total de requisições | 88.121      | 88.051          |
| Sucesso              | 100%        | 100%            |

Rust teve menor latência média e maior estabilidade, enquanto Java apresentou picos menores.

---

## 2. Uso de CPU

| Métrica     | Rust (axum) | Java (Undertow) |
|-------------|-------------|-----------------|
| CPU média   | 3,86%       | 3,78%           |
| CPU máxima  | 21,9%       | 21,9%           |

Virtualmente equivalentes em uso de CPU.

---

## 3. Uso de memória

| Métrica       | Rust (axum) | Java (Undertow) |
|---------------|-------------|-----------------|
| Memória média | 9.966 MB    | 10.076 MB       |
| Variação (σ)  | 39 MB       | 24 MB           |

Rust consome ligeiramente menos memória, enquanto Java apresentou menor variabilidade.

---

## 4. Tamanho do artefato

| Artefato        | Tamanho               |
|-----------------|-----------------------|
| Rust (axum)     | 877 KB (binário único) |
| Java (Undertow) | 3,6 MB (JAR com deps) |

O binário Rust é quase 4x menor e autossuficiente, enquanto o Java depende da JVM.

---

## Conclusões

1. Latência: Rust entrega menor tempo médio e maior estabilidade.  
2. Picos: Java suportou picos menores de latência.  
3. CPU: Empate técnico.  
4. Memória: Rust mais econômico, Java mais estável.  
5. Distribuição: Rust vence com artefato enxuto e sem runtime externo.

---

Resumo:  
- Rust é ideal quando eficiência máxima e distribuição simples são prioridades.  
- Java perde em latência e estabilidade, e exige JVM e produz artefatos maiores.

# O que eu realmente penso...

Este exemplo é muito simples, portanto, a comparação pode conter viés. Eu já rodei comparações com implementações mais complexas e pude notar que a vantagem do **Rust** aumenta muito. Mas quis manter o mesmo exemplo que usei até agora. Futuramente, montarei um exemplo mais complexo. De qualquer forma, este benchmark é bem mais preciso: https://programming-language-benchmarks.vercel.app/rust-vs-java

