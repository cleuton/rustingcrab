<img src="../guia_apis_rust.jpg" height=400>

---

<img src="../../../rusting-crab-logo.png" height=300>

---

# Guia das APIs em Rust

[**VER NO GITHUB**](https://github.com/cleuton/rustingcrab/tree/main/code_samples/guia_apis_rust/api_rest)

## REST

<a href="https://github.com/cleuton/rustingcrab">Guia das APIs em Rust e todo código-fonte incluído</a> © 2025 by <a href="https://github.com/cleuton">Cleuton Sampaio</a> is licensed under <a href="https://creativecommons.org/licenses/by-sa/4.0/">CC BY-SA 4.0</a><img src="https://mirrors.creativecommons.org/presskit/icons/cc.svg" alt="" style="max-width: 1em;max-height:1em;margin-left: .2em;"><img src="https://mirrors.creativecommons.org/presskit/icons/by.svg" alt="" style="max-width: 1em;max-height:1em;margin-left: .2em;"><img src="https://mirrors.creativecommons.org/presskit/icons/sa.svg" alt="" style="max-width: 1em;max-height:1em;margin-left: .2em;">

Este é um **ebook** interativo, com código-fonte disponível para você utilizar à vontade. Leia a licença de uso. 

[**Retornar ao menu**](../)

Em **Rust** podemos utilizar vários `crates` para criar uma **API Rest**, porém a forma mais simples é com `Axum` e `Tokyo`. `Axum` é um framework web moderno para Rust que facilita a criação de APIs REST com segurança de tipo, controle de erros robusto e integração nativa com o sistema de tipos do Rust. Ele é construído sobre `Tokio`, um runtime assíncrono essencial para lidar com alta concorrência com baixo uso de recursos, permitindo que as APIs respondam a muitas requisições simultâneas de forma eficiente. Juntos, Axum e Tokio formam uma combinação poderosa para desenvolver APIs rápidas, seguras e escaláveis, aproveitando o modelo assíncrono do Rust e sua garantia de segurança em tempo de compilação.

Primeiramente, mostrarei uma versão completa, mas sem a segurança e depois uma versão com `tsl` e `jwt`. 

### Exemplo simples de API

Vejamos a API que usamos como exemplo inicial: 

```toml
[package]
name = "api_simples"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1", features = ["full"] }
axum = "0.6"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

As dependências do projeto são bibliotecas que permitem construir uma API web em Rust:

- `tokio = { version = "1", features = ["full"] }`: fornece o runtime assíncrono necessário para executar tarefas simultâneas, como lidar com várias requisições ao mesmo tempo; a funcionalidade "full" inclui todos os componentes, como TCP, tempo e sincronização.
- `axum = "0.6"`: é o framework web usado para criar rotas, manipular requisições HTTP e responder com dados, sendo ideal para APIs REST e integrado nativamente com Tokio.
- `serde = { version = "1.0", features = ["derive"] }`: permite serializar e desserializar estruturas de dados, como converter entre structs do Rust e JSON; o recurso "derive" facilita isso com anotações como `#[derive(Serialize, Deserialize)]`.
- `serde_json = "1.0"`: trabalha junto com Serde para ler e escrever dados no formato JSON, comum em APIs web.

```rust
use axum::{
    routing::get,
    Json, Router,
};
use serde::Serialize;
use std::net::SocketAddr;

#[derive(Serialize)]
struct Item {
    id: u32,
    nome: String,
}

async fn listar_itens() -> Json<Vec<Item>> {
    let itens = vec![
        Item { id: 1, nome: "Maçã".into() },
        Item { id: 2, nome: "Banana".into() },
        Item { id: 3, nome: "Cereja".into() },
    ];
    Json(itens)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/itens", get(listar_itens));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Servidor rodando em http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

```

O código fonte define uma **API REST** simples usando `Axum` e `Tokio`. Ele expõe uma única rota que retorna uma lista de itens no formato **JSON**.

As linhas

```rust
use axum::{
    routing::get,
    Json, Router,
};
```

criam atalhos para itens do crate `axum`, permitindo referenciá-los diretamente sem escrever o caminho completo. Por exemplo, agora é possível usar `Router` em vez de `axum::Router`. O mesmo vale para `Json` e `get`: são disponibilizados no escopo atual para uso mais conveniente.

A linha

```rust
use serde::Serialize;
```

torna o trait `Serialize` acessível de forma direta, o que é necessário para a anotação `#[derive(Serialize)]` — ela gera automaticamente a implementação desse trait para a struct `Item`, permitindo que instâncias dela sejam convertidas para formatos como JSON.

A struct

```rust
struct Item {
    id: u32,
    nome: String,
}
```

representa um dado simples com um identificador numérico e um nome textual.

A função assíncrona

```rust
async fn listar_itens() -> Json<Vec<Item>> { ... }
```

retorna uma coleção fixa de três itens envolvida em `Json`, indicando ao Axum que o valor deve ser serializado como resposta HTTP em formato JSON.

Na função principal,

```rust
#[tokio::main]
async fn main() { ... }
```

a anotação `#[tokio::main]` configura o runtime assíncrono da Tokio como executor do programa, permitindo que funções `async` sejam executadas corretamente.

Dentro de `main`, cria-se um roteador com uma rota GET em `/itens` que chama a função `listar_itens`. O servidor é então ligado ao endereço `0.0.0.0:3000` e começa a aceitar conexões.

Quando alguém faz uma requisição para `/itens`, recebe a lista de frutas em JSON. O uso combinado de Axum, Tokio e Serde permite construir APIs leves, seguras e eficientes com poucas linhas de código.

### Uma API mais completa

Esta versão [**está na pasta "completa"**](./completa/).

Para termos um exemplo melhor, resolvi criar uma API mais completa, com os métodos HTTP básicos (GET, POST, PUT, DELETE) e um database. Vou usar o `SQLX` para acessar um database `PostgreSQL`.

O **SQLx** é uma biblioteca assíncrona para Rust que permite interagir diretamente com bancos de dados como PostgreSQL, MySQL, SQLite e MSSQL, sem depender de um ORM tradicional. Ele **não é um ORM completo**, mas sim uma camada de acesso a banco de dados que combina tipos seguros em tempo de compilação com consultas SQL puras.

Diferente de ORMs como ActiveRecord (Ruby) ou Hibernate (Java), o SQLx não mapeia automaticamente tabelas para objetos ou gera queries por trás. Em vez disso, você escreve SQL de verdade — o que dá mais controle — e o SQLx verifica os tipos das consultas **em tempo de compilação**, desde que o banco de dados esteja acessível durante a compilação (graças ao *compile-time query checking*).

As principais vantagens do SQLx são:

- **Sem macros complexas ou DSLs artificiais**: você usa SQL normal.
- **Segurança de tipo**: se uma coluna mudar no banco, o erro aparece na compilação, não em produção.
- **Alta performance**: sem sobrecarga de abstração, pois roda SQL direto.
- **Assíncrono nativo**: integrado com Tokio e runtime assíncrono do Rust.
- **Suporte a migrations**: inclui ferramentas para gerenciar scripts de migração do banco.

Em resumo, SQLx oferece um equilíbrio entre segurança, desempenho e controle, sendo ideal para quem quer evitar as armadilhas de ORMs pesados, mas ainda assim ter garantias fortes sobre as consultas, tudo dentro do estilo moderno e seguro do Rust.

Na [**pasta "completa"**](./completa/) temos essa versão da API. Para começar, temos o `SQL` de criação do database: 

```sql
-- init.sql
CREATE TABLE itens (
  id SERIAL PRIMARY KEY,
  nome TEXT NOT NULL,
  quantidade INT NOT NULL
);

INSERT INTO itens (nome, quantidade) VALUES
  ('Maçã', 10),
  ('Banana', 20),
  ('Cereja', 30);
```  

É bem simples e óbvio mesmo, para servir de base para qualquer projeto seu. Esse arquivo será utilizado pelo `podman` para subir um contêiner `postgres_db` com a tabela criada (e com alguns registros): 

```shell
podman run -d \
  --name postgres_db \
  -e POSTGRES_DB=postgres \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=password \
  -v "$PWD/init.sql":/docker-entrypoint-initdb.d/init.sql:Z \
  -p 5432:5432 \
  docker.io/library/postgres:latest
```

Eu mapeio o arquivo `init.sql` em um volume dentro do contêiner, do qual a imagem do postgres lerá para executar ao subir o database.

Finalmente, temos o script reescrito: 

```rust
use axum::{
    extract::{Extension, Path},
    routing::{get, post, put, delete},
    Json, Router, http::StatusCode,
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow};
use std::{env, net::SocketAddr};
use dotenvy::dotenv;

#[derive(Serialize, FromRow)]
struct Item {
    id: i32,
    nome: String,
    quantidade: i32,
}

#[derive(Deserialize)]
struct NovoItem {
    nome: String,
    quantidade: i32,
}

#[derive(Deserialize)]
struct AtualizaItem {
    nome: Option<String>,
    quantidade: Option<i32>,
}

async fn listar_itens(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<Item>>, StatusCode> {
    let itens = sqlx::query_as::<_, Item>("SELECT id, nome, quantidade FROM itens")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(itens))
}

async fn criar_item(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<NovoItem>,
) -> Result<(StatusCode, Json<Item>), StatusCode> {
    let item = sqlx::query_as::<_, Item>(
        "INSERT INTO itens (nome, quantidade) VALUES ($1, $2) RETURNING id, nome, quantidade"
    )
    .bind(&payload.nome)
    .bind(payload.quantidade)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((StatusCode::CREATED, Json(item)))
}

async fn atualizar_item(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<AtualizaItem>,
) -> Result<Json<Item>, StatusCode> {
    let item = sqlx::query_as::<_, Item>(
        "UPDATE itens SET
            nome      = COALESCE($1, nome),
            quantidade= COALESCE($2, quantidade)
         WHERE id = $3
         RETURNING id, nome, quantidade"
    )
    .bind(payload.nome)
    .bind(payload.quantidade)
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Json(item))
}

async fn deletar_item(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let resultado = sqlx::query!("DELETE FROM itens WHERE id = $1", id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if resultado.rows_affected() == 1 {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let url = env::var("DATABASE_URL")
        .expect("Não conseguiu conectar ao Postgres - DATABASE_URL não definido");
    let pool = PgPool::connect(&url)
        .await
        .expect("Não conseguiu conectar ao Postgres");

    let app = Router::new()
        .route("/itens", get(listar_itens).post(criar_item))
        .route("/itens/:id", put(atualizar_item).delete(deletar_item))
        .layer(Extension(pool));

    let addr = SocketAddr::from(([0,0,0,0], 3000));
    println!("API rodando em http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

Temos mais de uma função assíncrona para lidar com cada tipo de request: 

```rust
    let app = Router::new()
        .route("/itens", get(listar_itens).post(criar_item))
        .route("/itens/:id", put(atualizar_item).delete(deletar_item))
        .layer(Extension(pool));
```

E a novidade é o uso dos comandos `SQLX` para lidar com o database. 

#### Configuração

Precisamos configurar a API para acessar o database (`DATABASE_URL`) e isso é feito com variáveis de ambiente. O `crate` `dotenvy` faz isso. Ele procura um arquivo `.env` e cria a variável de ambiente para nós, depois do comando: `dotenv()`. O `ok()` descarta se der errou, ou seja, se o arquivo `.env` não existir (o que acontece em **produção**), pois espera-se que a variável de ambiente esteja criada. 

#### SQLX

Vamos focar **apenas nos comandos e usos do SQLx** no código, explicando de forma simples e objetiva o que cada um faz:

O tipo `PgPool` é um *pool de conexões* para PostgreSQL, criado fora das funções (no `main`) com `PgPool::connect(&url)`. Ele é compartilhado entre as rotas via `Extension(pool)` e usado em todas as consultas ao banco.

Dentro das rotas, o SQLx é usado assim:

### 1. `sqlx::query_as::<_, Item>("SELECT ...")`
- Executa uma consulta SQL e tenta mapear o resultado diretamente para a struct `Item`, graças ao atributo `#[derive(FromRow)]`.
- O `<_, Item>` indica que o segundo parâmetro genérico é o tipo alvo (`Item`), enquanto o primeiro (sublinhado) é inferido automaticamente.
- Exemplo:
  ```rust
  sqlx::query_as::<_, Item>("SELECT id, nome, quantidade FROM itens")
      .fetch_all(&pool)
      .await
  ```
  - Pega todos os itens da tabela `itens` e retorna como um `Vec<Item>`.

### 2. `.bind(...)`
- Substitui os placeholders (`$1`, `$2`, etc.) na query pelos valores fornecidos.
- Garante segurança contra injeção de SQL, pois os valores são passados separadamente do texto da query.
- Exemplo:
  ```rust
  .bind(&payload.nome)
  .bind(payload.quantidade)
  ```
  - Coloca `nome` em `$1` e `quantidade` em `$2`.

### 3. `.fetch_one(...)` vs `.fetch_all(...)`
- `fetch_one`: espera exatamente **uma linha** como resultado (erro se não encontrar).
  Usado em `criar_item` e `atualizar_item`, porque esperamos o item recém-inserido ou alterado.
- `fetch_all`: retorna **todas as linhas** da consulta.
  Usado em `listar_itens` para pegar todos os registros.

### 4. `sqlx::query!`
- Uma macro diferente: **verificada em tempo de compilação**.
- No caso de `deletar_item`:
  ```rust
  sqlx::query!("DELETE FROM itens WHERE id = $1", id)
      .execute(&pool)
      .await
  ```
  - Executa um comando que não retorna linhas (como DELETE), apenas informações sobre a execução.
- Diferente de `query_as`, aqui não há conversão para uma struct, mas ainda há verificação de tipos: o compilador checa se `$1` é do tipo correto (aqui, `i32`), comparando com a estrutura do banco.

### 5. `.execute(...)`
- Usado com `sqlx::query!` para comandos que modificam dados (INSERT, UPDATE, DELETE).
- Retorna um objeto com metadados, como `rows_affected()` — útil para saber quantas linhas foram deletadas.

### Resumo dos comandos SQLx:
- `sqlx::query_as`: roda uma query e converte os resultados em structs (com `FromRow`).
- `.bind()`: insere valores de forma segura nos parâmetros da query (`$1`, `$2`...).
- `.fetch_one()`: obtém um único resultado.
- `.fetch_all()`: obtém todos os resultados.
- `sqlx::query!`: macro para queries verificadas em tempo de compilação (tipo seguro).
- `.execute()`: executa comandos que não retornam linhas, como DELETE.

#### Teste

O arquivo `teste.txt` tem todos os comandos para você subir o database e testar a nova versão da **API**.

### Agora uma versão mais segura

Esta versão está na [**pasta "segurança"**](./seguranca/).

Para termos um mínimo de segurança, precisamos de 2 coisas: `TSL` (SSL) e controle de acesso (JWT). Vou gerar um certificado auto-assinado e implementar HTTPS e uso de tokens JWT na aplicação. 

> Um **certificado auto‑assinado** é uma chave pública criptograficamente ligada a uma entidade, gerada e assinada por ela mesma em vez de por uma Autoridade Certificadora reconhecida, o que permite habilitar HTTPS com criptografia TLS para testes, desenvolvimento ou ambientes internos sem custo, mas exige que o cliente (navegador ou aplicação) seja configurado para confiar explicitamente nesse certificado, já que, por não ter uma cadeia de confiança pública, ele não é aceito automaticamente em produção, garantindo confidencialidade dos dados em trânsito mesmo em cenários controlados.

> O **JWT** (JSON Web Token) é um token auto­contido que leva dentro de si “claims” (declarações) sobre a identidade e permissões do usuário, assinado digitalmente pelo servidor para garantir autenticidade e integridade; após autenticação, o servidor emite um JWT que o cliente inclui em cada requisição (no cabeçalho Authorization ou em cookie HttpOnly), permitindo ao backend validar o token sem manter estado de sessão, extrair as claims e liberar ou negar acesso a recursos segundo papéis, escopos ou regras definidas — uma abordagem leve e escalável, mas que exige bom gerenciamento de expiração e estratégias de revogação.

#### Gerando o certificado

Dentro do arquivo [`testes.txt`](./seguranca/testes.txt), temos o comando do `openssl` para gerar os dois arquivos: 

```shell
openssl req -x509 -newkey rsa:4096 -nodes \
  -keyout key.pem -out cert.pem -days 365 \
  -subj "/CN=localhost"
```

Este comando vai criar dois arquivos: `cert.pem` (certificado com a chave pública) e `key.pem` (chave primária) na pasta do projeto.

#### Lendo o segredo da variável de ambiente

No arquivo `.env` criei mais uma variável: 

```text
JWT_SECRET=meu-segredo-pavoroso
```

A variável de ambiente **JWT_SECRET** serve como a chave secreta usada para assinar e verificar tokens JWT (JSON Web Tokens). Ela é essencial para garantir que:

- Apenas quem conhece o segredo possa gerar ou alterar tokens válidos.
- O servidor possa validar que um token recebido não foi falsificado.

E no [**código novo**](./seguranca/src/main.rs) eu uso essa variável em dois momentos: 

```rust
// Login:
let chave = env::var("JWT_SECRET").expect("JWT_SECRET não definido");
// validar token: 
let chave = env::var("JWT_SECRET").expect("JWT_SECRET não definido");
```

#### Rota de login

A rota de login recebe um JSON com `usuario` e `senha`. Se forem "admin" e "password", o servidor gera um token JWT usando a chave secreta `JWT_SECRET`, define a expiração em 1 hora e envia esse token no cabeçalho `Set-Cookie`, como um cookie chamado `token`. O cookie é marcado como `HttpOnly` e `Secure`, o que o torna inacessível via JavaScript e só enviado por HTTPS. Se as credenciais estiverem erradas, retorna erro 401.

```rust
// Rota de login: gera JWT e envia em Set-Cookie
async fn login(Json(cred): Json<Credenciais>) -> impl IntoResponse {
    if cred.usuario == "admin" && cred.senha == "password" {
        let chave = env::var("JWT_SECRET").expect("JWT_SECRET não definido");
        let exp = (Utc::now().timestamp() + 3600) as usize;
        let claims = Claims { sub: cred.usuario, exp };
        let token = encode(&JwtHeader::default(), &claims, &EncodingKey::from_secret(chave.as_bytes()))
            .unwrap();

        let cookie = format!(
            "token={}; HttpOnly; Secure; Path=/; Max-Age=3600",
            token
        );

        let mut headers = HeaderMap::new();
        headers.insert("set-cookie", HeaderValue::from_str(&cookie).unwrap());
        (StatusCode::OK, headers, Json("Logado com sucesso"))
    } else {
        (StatusCode::UNAUTHORIZED, HeaderMap::new(), Json("Credenciais inválidas"))
    }
}
```

Exemplo de login: 

```shell
curl -i -k -X POST \
-H "Content-type: application/json" \
-d '{"usuario" : "admin", "senha" : "password"}' \
https://localhost:3000/login
HTTP/2 200 
content-type: application/json
set-cookie: token=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJhZG1pbiIsImV4cCI6MTc1MzIwMDA1OH0.lPiY5og_QyrQ78MMtNeCfQCDVW2dJC4ZiZIt0203dbg; HttpOnly; Secure; Path=/; Max-Age=3600
content-length: 20
date: Tue, 22 Jul 2025 15:00:58 GMT

"Logado com sucesso"
```

Para rodar os outros testes, copie do header "set-cookie" apenas o "token=eyJ0..." e cole em "\<seu token\>".

É fundamental usarmos o atributo de linha de comando `-k` no `curl`, pois estamos utilizando certificado **auto-assinado**.

#### Validação de token

Se você fizer qualquer outro request, o token será validado: 

```rust
async fn listar_itens(
    TypedHeader(cookies): TypedHeader<Cookie>,
    Extension(pool): Extension<Arc<PgPool>>,
) -> Result<Json<Vec<Item>>, StatusCode> {
    validar_token(&cookies)?;
    let itens = sqlx::query_as::<_, Item>("SELECT id, nome, quantidade FROM itens")
        .fetch_all(&*pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(itens))
}
```

A função `validar_token` valida a assinatura e decodifica os `claims`. No nosso caso temos:`sub` (subject ou usuário) e `exp` (data de expiração do token).

Se você tentar fazer um acesso sem token, tomará esse erro: 

```shell
curl -i -k -X GET https://localhost:3000/itens
HTTP/2 401 
content-length: 0
date: Tue, 22 Jul 2025 15:06:23 GMT
```

Para acessar corretamente é preciso passar o `cookie` com o `token` recebido após o **login**: 

```shell
curl -i -k -X GET \
--cookie "token=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJhZG1pbiIsImV4cCI6MTc1MzIwMDA1OH0.lPiY5og_QyrQ78MMtNeCfQCDVW2dJC4ZiZIt0203dbg" \
https://localhost:3000/itens
HTTP/2 200 
content-type: application/json
content-length: 124
date: Tue, 22 Jul 2025 15:07:50 GMT

[{"id":1,"nome":"Maçã","quantidade":10},{"id":2,"nome":"Banana","quantidade":20},{"id":3,"nome":"Cereja","quantidade":30}]
```

O resto dos testes deve passar sem problemas. 

[**Retornar ao menu**](../)



