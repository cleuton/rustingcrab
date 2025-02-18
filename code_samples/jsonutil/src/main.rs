use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

/// Retorna o JSON formatado (pretty-print)
fn pretty_print(json_data: &Value) -> String {
    serde_json::to_string_pretty(json_data).unwrap()
}

/// Atualiza um atributo em um objeto JSON (se o atributo existir)
fn update_attribute(json_data: &mut Value, attr: &str, new_value: Value) -> bool {
    if let Value::Object(map) = json_data {
        if map.contains_key(attr) {
            map.insert(attr.to_string(), new_value);
            return true;
        }
    }
    false
}

/// Insere um novo atributo em um objeto JSON (mesmo que já exista, sobreescreve)
fn insert_attribute(json_data: &mut Value, attr: &str, new_value: Value) {
    if let Value::Object(map) = json_data {
        map.insert(attr.to_string(), new_value);
    }
}

/// Remove um atributo de um objeto JSON. Retorna o valor removido, se existir.
fn remove_attribute(json_data: &mut Value, attr: &str) -> Option<Value> {
    if let Value::Object(map) = json_data {
        map.remove(attr)
    } else {
        None
    }
}

/// Estado compartilhado da aplicação: nosso JSON, inicialmente vazio.
#[derive(Clone)]
struct AppState {
    data: Arc<Mutex<Value>>,
}

/// Payload para configurar o JSON via POST
#[derive(Deserialize)]
struct SetJsonPayload {
    json: Value,
}

/// Endpoint para configurar o JSON via POST
async fn set_json(
    State(state): State<AppState>,
    Json(payload): Json<SetJsonPayload>,
) -> impl IntoResponse {
    let mut data = state.data.lock().unwrap();
    *data = payload.json;
    // Retorna o JSON atual em pretty_print
    (StatusCode::OK, pretty_print(&data))
}

/// Endpoint para consultar o JSON atual
async fn get_json(State(state): State<AppState>) -> impl IntoResponse {
    let data = state.data.lock().unwrap();
    (StatusCode::OK, pretty_print(&data))
}

/// Payload para atualizar um atributo
#[derive(Deserialize)]
struct UpdatePayload {
    attribute: String,
    value: Value,
}

/// Endpoint para atualizar um atributo do JSON
async fn update_attribute_handler(
    State(state): State<AppState>,
    Json(payload): Json<UpdatePayload>,
) -> impl IntoResponse {
    let mut data = state.data.lock().unwrap();
    if update_attribute(&mut *data, &payload.attribute, payload.value) {
        (StatusCode::OK, pretty_print(&data))
    } else {
        (StatusCode::BAD_REQUEST, pretty_print(&data))
    }
}

/// Payload para inserir um atributo
#[derive(Deserialize)]
struct InsertPayload {
    attribute: String,
    value: Value,
}

/// Endpoint para inserir um atributo no JSON
async fn insert_attribute_handler(
    State(state): State<AppState>,
    Json(payload): Json<InsertPayload>,
) -> impl IntoResponse {
    let mut data = state.data.lock().unwrap();
    insert_attribute(&mut *data, &payload.attribute, payload.value);
    (StatusCode::OK, pretty_print(&data))
}

/// Payload para remover um atributo
#[derive(Deserialize)]
struct RemovePayload {
    attribute: String,
}

/// Endpoint para remover um atributo do JSON
async fn remove_attribute_handler(
    State(state): State<AppState>,
    Json(payload): Json<RemovePayload>,
) -> impl IntoResponse {
    let mut data = state.data.lock().unwrap();
    let _ = remove_attribute(&mut *data, &payload.attribute);
    (StatusCode::OK, pretty_print(&data))
}

/// Payload para inserir um atributo aninhado (dentro de um objeto existente)
#[derive(Deserialize)]
struct InsertNestedPayload {
    parent: String,
    attribute: String,
    value: Value,
}

/// Endpoint para inserir um atributo em um objeto aninhado
async fn insert_nested_handler(
    State(state): State<AppState>,
    Json(payload): Json<InsertNestedPayload>,
) -> impl IntoResponse {
    let mut data = state.data.lock().unwrap();
    if let Some(parent_obj) = data.get_mut(&payload.parent) {
        if let Value::Object(ref mut map) = parent_obj {
            map.insert(payload.attribute, payload.value);
            return (StatusCode::OK, pretty_print(&data));
        }
    }
    (StatusCode::BAD_REQUEST, pretty_print(&data))
}

#[tokio::main]
async fn main() {
    // Estado inicial: objeto JSON vazio
    let state = AppState {
        data: Arc::new(Mutex::new(json!({}))),
    };

    // Define as rotas da API
    let app = Router::new()
        .route("/json/set", post(set_json))
        .route("/json", get(get_json))
        .route("/json/update", post(update_attribute_handler))
        .route("/json/insert", post(insert_attribute_handler))
        .route("/json/remove", delete(remove_attribute_handler))
        .route("/json/insert_nested", post(insert_nested_handler))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
