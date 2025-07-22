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
