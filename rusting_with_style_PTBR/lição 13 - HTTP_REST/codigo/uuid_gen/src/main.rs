use tiny_http::{Server, Response, Method, Header};
use serde_json::json;
use uuid::Uuid;

fn main() {
    // Inicia o servidor HTTP na porta 3030
    let server = Server::http("127.0.0.1:3030").unwrap();
    println!("Servidor rodando em http://127.0.0.1:3030");

    // Loop infinito para aceitar e processar requisições
    for request in server.incoming_requests() {
        match (request.method(), request.url()) {
            // Rota GET /uuid
            (&Method::Get, "/uuid") => {
                // Gera um novo UUID v4
                let new_uuid = Uuid::new_v4();

                // Constrói o JSON de resposta
                let response_body = serde_json::to_string(&json!({
                    "uuid": new_uuid.to_string()
                })).unwrap();

                // Monta e envia a resposta (status 200, content-type JSON)
                let response = Response::from_string(response_body)
                    .with_header(Header::from_bytes("Content-Type", "application/json").unwrap())
                    .with_status_code(200);
                let _ = request.respond(response);
            },
            // Se não for /uuid, retorna 404
            _ => {
                let response = Response::from_string("Not Found").with_status_code(404);
                let _ = request.respond(response);
            }
        }
    }
}

