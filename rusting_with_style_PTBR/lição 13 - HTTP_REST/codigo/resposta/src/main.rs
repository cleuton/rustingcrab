use std::sync::Arc;
use std::thread;

use tiny_http::{Server, Response, Method, Header};
use serde_json::json;

// Importa nossa lógica de DB
use crate::db::listar_pessoas;

// Precisamos declarar o módulo 'db' para sabermos onde está
mod db;

fn main() {
    // Inicia o servidor (tiny_http) em 127.0.0.1:3030
    let server = Arc::new(Server::http("127.0.0.1:3030").unwrap());
    println!("Servidor rodando em http://127.0.0.1:3030");

    // Vamos criar 4 threads para processamento
    let num_threads = 4;

    for i in 0..num_threads {
        let server_clone = Arc::clone(&server);

        thread::spawn(move || {
            println!("Thread {} iniciada", i);

            // Cada thread fica pegando requisições (loop infinito)
            for request in server_clone.incoming_requests() {
                let method = request.method();
                let path = request.url();

                match (method, path) {
                    // Rota GET /pessoas
                    (&Method::Get, "/pessoas") => {
                        // 1. Chama função que retorna Vec<Pessoa> do DB
                        let lista = listar_pessoas();

                        // 2. Serializa a lista de pessoas em JSON
                        let json_resp = match serde_json::to_string(&lista) {
                            Ok(j) => j,
                            Err(e) => {
                                eprintln!("Erro ao serializar: {:?}", e);
                                String::from("[]")
                            }
                        };

                        // 3. Monta a resposta HTTP com o JSON
                        let response = Response::from_string(json_resp)
                            .with_header(Header::from_bytes("Content-Type", "application/json").unwrap())
                            .with_status_code(200);
                        let _ = request.respond(response);
                    }

                    // Qualquer outra rota → 404
                    _ => {
                        let response = Response::from_string("Not Found").with_status_code(404);
                        let _ = request.respond(response);
                    }
                }
            }
        });
    }

    // Mantém a main "viva" para que as threads continuem rodando
    loop {
        thread::park();
    }
}
