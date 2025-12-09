![](logo.png)

[**REPO**](https://github.com/cleuton/rustingcrab/tree/main/code_samples/opentelemetry)

[**Cleuton Sampaio**](https://linkedin.com/in/cleutonsampaio)

[**ENGLISH**](./ENGLISH.md)

# jaeger-http-demo

Demonstração de tracing distribuído em Rust usando OpenTelemetry, OTLP/HTTP e Jaeger.

Este projeto implementa um exemplo minimalista porém realista de tracing distribuído: uma requisição inicia no “frontend” e é propagada para um “ads-service”, mantendo o mesmo trace e span context. Isso permite visualizar toda a cadeia da requisição dentro do Jaeger.

---

## O que está sendo demonstrado

Este exemplo cobre os elementos fundamentais do ecossistema OpenTelemetry:

### 1. Instrumentação manual no código

O projeto cria spans manualmente utilizando `tracing` + `tracing-opentelemetry`, permitindo capturar eventos, atributos e relações parent-child entre spans.

### 2. Exportação via OTLP (OpenTelemetry Protocol)

Utilizamos o protocolo **OTLP** no modo **HTTP** para enviar os traces diretamente ao Jaeger, sem precisar do OpenTelemetry Collector.
Endpoint usado:

```
http://localhost:4318/v1/traces
```

### 3. Propagação de contexto entre serviços

O exemplo demonstra distributed tracing real, criando spans em um serviço e propagando o contexto para outro, simulando uma requisição remota.

### 4. Visualização no Jaeger

Toda a estrutura de spans, atributos e hierarquia pode ser visualizada na UI do Jaeger.

---

## Arquitetura

```
frontend (span root)
     │
     ▼
ads-service (span filho)
```

Cada parte registra seus próprios eventos e atributos, mas sempre vinculados ao mesmo trace.

---

## Como executar

### 1. Subir o Jaeger com OTLP habilitado

Use apenas Docker. Sem Docker Compose.

```bash
docker run \
  -e COLLECTOR_OTLP_ENABLED=true \
  -p 16686:16686 \
  -p 4317:4317 \
  -p 4318:4318 \
  jaegertracing/all-in-one:latest
```

Depois abra:

```
http://localhost:16686
```

---

### 2. Rodar o exemplo Rust

```bash
cargo run
```

Você verá logs do tipo:

```
frontend: received request
ads-service: processing ads data
```

Se o Jaeger estiver rodando, nada quebra e os spans aparecem.

---

## Conferindo no Jaeger

Na UI, procure pelo serviço:

```
jaeger-http-demo
```

Você verá:

* o span “frontend”
* o span “ads-service”
* hierarquia parent → child
* atributos
* tempo de execução
* timeline

![](jaeger.png)

---

## Como funciona o OTLP

OTLP é o protocolo nativo do OpenTelemetry. Neste exemplo usamos a versão HTTP, enviando spans diretamente para o Jaeger no endpoint `/v1/traces`.

Sem Collector intermediário.
Jaeger recebe e mostra.

---

## Código chave

### iniciar tracer

```rust
let tracer = global::tracer("jaeger-http-demo");
```

### criar span principal

```rust
let root_span = span!(Level::INFO, "frontend", user_id = 42);
```

### propagar contexto

```rust
prop.inject_context(&cx, &mut injector);
```

### recuperar contexto no outro serviço

```rust
prop.extract(&extractor)
```

---

## Por que isso é distributed tracing

Porque:

* cada serviço cria spans independentes
* o contexto é propagado entre serviços
* o Jaeger reconhece que tudo pertence ao mesmo trace
* você enxerga o fluxo ponta-a-ponta

Ou seja, é exatamente o conceito de distributed tracing.

---

## Próximos Passos

Você pode:

* adicionar serviços reais (axum, tonic, reqwest)
* instrumentação automática (`#[instrument]`)
* métricas
* logs
* OpenTelemetry Collector
* Propagadores diferentes (W3C, B3, Jaeger)

Esse projeto já serve como base para aplicações distribuídas reais.
