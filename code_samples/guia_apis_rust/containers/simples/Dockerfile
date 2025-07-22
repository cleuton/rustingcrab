# --- Stage 1: compilação ---
FROM docker.io/library/rust:latest AS construtor

# define o diretório de trabalho
WORKDIR /usr/src/api_simples

# copia o Cargo.toml e o Cargo.lock e faz o "fetch" das crates
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main(){}" > src/main.rs
RUN cargo fetch

# agora copia o código-fonte e compila em release
COPY src/ src/
RUN cargo build --release

# --- Stage 2: runtime ---
FROM docker.io/library/debian:bookworm-slim

# instala certificados TLS (se sua API chamar HTTPS externamente)
RUN apt-get update \
 && apt-get install -y ca-certificates \
 && rm -rf /var/lib/apt/lists/*

# define diretório onde ficará o binário
WORKDIR /usr/local/bin

# copia o binário otimizado do stage anterior
COPY --from=construtor /usr/src/api_simples/target/release/api_simples .

# expõe a porta que a API usa
EXPOSE 3000

# comando de início
ENTRYPOINT ["./api_simples"]
