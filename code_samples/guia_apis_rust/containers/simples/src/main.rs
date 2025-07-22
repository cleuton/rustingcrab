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
