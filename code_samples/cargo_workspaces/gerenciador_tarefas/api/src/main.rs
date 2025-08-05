use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use core::GerenciadorDeTarefas;
use std::sync::Mutex;

async fn listar_tarefas(dados: web::Data<Mutex<GerenciadorDeTarefas>>) -> impl Responder {
    let gerenciador = dados.lock().unwrap();
    let tarefas = gerenciador.listar_tarefas();
    HttpResponse::Ok().json(tarefas)
}

async fn adicionar_tarefa(
    dados: web::Data<Mutex<GerenciadorDeTarefas>>,
    tarefa: web::Json<core::Tarefa>,
) -> impl Responder {
    let mut gerenciador = dados.lock().unwrap();
    gerenciador.adicionar_tarefa(tarefa.into_inner());
    HttpResponse::Created().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let gerenciador = web::Data::new(Mutex::new(GerenciadorDeTarefas::novo()));

    println!("Servidor rodando em http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(gerenciador.clone())
            .route("/tarefas", web::get().to(listar_tarefas))
            .route("/tarefas", web::post().to(adicionar_tarefa))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
