mod tarefa {
    tonic::include_proto!("tarefa");
}

use tarefa::tarefa_service_server::{TarefaService, TarefaServiceServer};
use tarefa::{CriarTarefaRequest, ListaTarefasRequest, ListaTarefasResponse, Tarefa};
use tonic::{transport::Server, Request, Response, Status};
use std::{net::SocketAddr, sync::{Arc, Mutex}};

#[derive(Default)]
struct ServidorTarefa {
    tarefas: Arc<Mutex<Vec<Tarefa>>>,
}

#[tonic::async_trait]
impl TarefaService for ServidorTarefa {
    async fn listar(
        &self,
        _req: Request<ListaTarefasRequest>,
    ) -> Result<Response<ListaTarefasResponse>, Status> {
        let guard = self.tarefas.lock().unwrap();
        let resp = ListaTarefasResponse { tarefas: guard.clone() };
        Ok(Response::new(resp))
    }

    async fn criar(
        &self,
        req: Request<CriarTarefaRequest>,
    ) -> Result<Response<Tarefa>, Status> {
        let mut guard = self.tarefas.lock().unwrap();
        let novo_id = (guard.len() as i32) + 1;
        let descricao = req.into_inner().descricao;
        let tarefa = Tarefa { id: novo_id, descricao, concluida: false };
        guard.push(tarefa.clone());
        Ok(Response::new(tarefa))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let endereco: SocketAddr = "[::1]:50051".parse()?;
    let servidor = ServidorTarefa::default();

    println!("gRPC rodando em {}", endereco);
    Server::builder()
        .add_service(TarefaServiceServer::new(servidor))
        .serve(endereco)
        .await?;
    Ok(())
}
