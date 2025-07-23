// Copyright 2023 Cleuton Sampaio.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use axum::{
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::Serialize;
use sonyflake::Sonyflake;
use std::sync::Arc;
use tokio::sync::OnceCell;

const VERSION: &str = "0.1.0";

#[derive(Serialize)]
struct IdResponse {
    error: bool,
    id: Option<u64>,
}

// Usar OnceCell para inicialização lazy e thread-safe
static FLAKE: OnceCell<Arc<Sonyflake>> = OnceCell::const_new();

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    println!("INFO: ZAPTIDGEN RUST - V {} - starting... (port: 8888)", VERSION);

    // Inicializar Sonyflake apenas uma vez
    let flake = Arc::new(
        Sonyflake::new().expect("FATAL ERROR: Couldn't create Sonyflake")
    );
    
    // Ignorar o erro de Debug usando unwrap_or_else
    FLAKE.set(flake).unwrap_or_else(|_| panic!("Flake already initialized"));
    
    let app = Router::new()
        .route("/nextid", get(next_id_handler));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8888")
        .await
        .expect("FATAL ERROR: failed to bind to address");

    println!("INFO: server listening at 0.0.0.0:8888");
    
    axum::serve(listener, app)
        .await
        .expect("FATAL ERROR: failed to serve");
}

async fn next_id_handler() -> Result<Json<IdResponse>, (StatusCode, Json<IdResponse>)> {
    let flake = FLAKE.get().expect("Flake not initialized");
    
    match flake.next_id() {
        Ok(id) => Ok(Json(IdResponse {
            error: false,
            id: Some(id),
        })),
        Err(e) => {
            eprintln!("ERROR: flake.next_id() failed with {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(IdResponse {
                    error: true,
                    id: None,
                }),
            ))
        }
    }
}