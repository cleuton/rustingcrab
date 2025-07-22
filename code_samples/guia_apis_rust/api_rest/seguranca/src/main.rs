use axum::{
    extract::{TypedHeader, Extension, Path},
    http::{HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::{get, post, put, delete},
    Json, Router,
};
use axum_server::tls_rustls::RustlsConfig;
use chrono::Utc;
use dotenvy::dotenv;
use headers::Cookie;
use jsonwebtoken::{encode, decode, Header as JwtHeader, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use std::{env, net::SocketAddr, sync::Arc};

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

#[derive(Deserialize)]
struct Credenciais {
    usuario: String,
    senha: String,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

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

// Valida o JWT extraindo-o do cookie TypedHeader<Cookie>
fn validar_token(cookies: &Cookie) -> Result<Claims, StatusCode> {
    let token = cookies.get("token").ok_or(StatusCode::UNAUTHORIZED)?;
    let chave = env::var("JWT_SECRET").expect("JWT_SECRET não definido");
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(chave.as_bytes()),
        &Validation::default(),
    ).map_err(|_| StatusCode::UNAUTHORIZED)?;
    Ok(data.claims)
}

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

async fn criar_item(
    TypedHeader(cookies): TypedHeader<Cookie>,
    Extension(pool): Extension<Arc<PgPool>>,
    Json(payload): Json<NovoItem>,
) -> Result<(StatusCode, Json<Item>), StatusCode> {
    validar_token(&cookies)?;
    let item = sqlx::query_as::<_, Item>(
        "INSERT INTO itens (nome, quantidade) VALUES ($1, $2) RETURNING id, nome, quantidade",
    )
    .bind(&payload.nome)
    .bind(payload.quantidade)
    .fetch_one(&*pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((StatusCode::CREATED, Json(item)))
}

async fn atualizar_item(
    TypedHeader(cookies): TypedHeader<Cookie>,
    Extension(pool): Extension<Arc<PgPool>>,
    Path(id): Path<i32>,
    Json(payload): Json<AtualizaItem>,
) -> Result<Json<Item>, StatusCode> {
    validar_token(&cookies)?;
    let item = sqlx::query_as::<_, Item>(
        "UPDATE itens
         SET nome = COALESCE($1, nome),
             quantidade = COALESCE($2, quantidade)
         WHERE id = $3
         RETURNING id, nome, quantidade",
    )
    .bind(payload.nome)
    .bind(payload.quantidade)
    .bind(id)
    .fetch_one(&*pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Json(item))
}

async fn deletar_item(
    TypedHeader(cookies): TypedHeader<Cookie>,
    Extension(pool): Extension<Arc<PgPool>>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    validar_token(&cookies)?;
    let res = sqlx::query!("DELETE FROM itens WHERE id = $1", id)
        .execute(&*pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if res.rows_affected() == 1 {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL não definido");
    let pool = Arc::new(
        PgPool::connect(&url)
            .await
            .expect("Falha ao conectar ao Postgres"),
    );

    // carrega cert.pem e key.pem
    let tls = RustlsConfig::from_pem_file("cert.pem", "key.pem")
        .await
        .expect("Falha ao carregar cert.pem/key.pem");

    let app = Router::new()
        .route("/login", post(login))
        .route("/itens", get(listar_itens).post(criar_item))
        .route("/itens/:id", put(atualizar_item).delete(deletar_item))
        .layer(Extension(pool));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("API HTTPS+JWT rodando em https://{}", addr);

    axum_server::bind_rustls(addr, tls)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
