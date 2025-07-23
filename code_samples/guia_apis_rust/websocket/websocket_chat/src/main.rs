use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::sync::broadcast;

// Estado compartilhado da aplicação
type Usuarios = Arc<RwLock<HashMap<String, broadcast::Sender<MensagemServidor>>>>;

// Estrutura para mensagens de chat
#[derive(Debug, Clone, Serialize, Deserialize)]
struct MensagemChat {
    usuario: String,
    texto: String,
    horario: String,
}

// Estrutura para diferentes tipos de mensagens do servidor
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "tipo")]
enum MensagemServidor {
    UsuarioEntrou { usuario: String },
    UsuarioSaiu { usuario: String },
    MensagemChat(MensagemChat),
}

#[tokio::main]
async fn main() {
    // Armazenamento em memória dos canais de usuários conectados
    let usuarios: Usuarios = Arc::new(RwLock::new(HashMap::new()));

    // Criar rota para WebSocket
    let app = Router::new()
        .route("/ws", get(websocket_handler))
        .with_state(usuarios);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3030")
        .await
        .unwrap();
        
    println!("Servidor WebSocket rodando em ws://127.0.0.1:3030/ws");
    axum::serve(listener, app).await.unwrap();
}

// Handler para conexão WebSocket
async fn websocket_handler(
    ws: WebSocketUpgrade,
    axum::extract::State(usuarios): axum::extract::State<Usuarios>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, usuarios))
}

// Função para tratar a conexão de um novo cliente
async fn handle_socket(socket: WebSocket, usuarios: Usuarios) {
    let (mut sender, mut receiver) = socket.split();
    
    // Esperar o nome de usuário do cliente
    let nome_usuario = loop {
        if let Some(Ok(message)) = receiver.next().await {
            if let Message::Text(texto) = message {
                if let Ok(msg) = serde_json::from_str::<serde_json::Value>(&texto) {
                    if msg["tipo"] == "entrada" {
                        if let Some(nome) = msg["usuario"].as_str() {
                            break nome.to_string();
                        }
                    }
                }
            }
        } else {
            return;
        }
    };

    // Criar um canal broadcast para este usuário
    let (tx, mut rx) = broadcast::channel(100);
    
    // Adicionar usuário à lista de usuários conectados
    usuarios.write().unwrap().insert(nome_usuario.clone(), tx.clone());

    // Notificar todos que um novo usuário entrou
    let msg_entrada = MensagemServidor::UsuarioEntrou {
        usuario: nome_usuario.clone(),
    };
    enviar_mensagem_todos(&usuarios, msg_entrada);

    // Clonar referências para as tarefas
    let usuarios_clone1 = usuarios.clone();
    let usuarios_clone2 = usuarios.clone();

    // Tarefa para enviar mensagens para o usuário
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if let Ok(texto) = serde_json::to_string(&msg) {
                if sender.send(Message::Text(texto)).await.is_err() {
                    break;
                }
            }
        }
    });

    // Tarefa para receber mensagens do usuário
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(message)) = receiver.next().await {
            if let Message::Text(texto) = message {
                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&texto) {
                    if parsed["tipo"] == "mensagem" {
                        if let (Some(usuario), Some(mensagem)) = (
                            parsed["usuario"].as_str(),
                            parsed["texto"].as_str(),
                        ) {
                            let msg_chat = MensagemServidor::MensagemChat(MensagemChat {
                                usuario: usuario.to_string(),
                                texto: mensagem.to_string(),
                                horario: chrono::Utc::now().to_rfc3339(),
                            });
                            enviar_mensagem_todos(&usuarios_clone1, msg_chat);
                        }
                    }
                }
            }
        }
    });

    // Aguardar qualquer tarefa terminar
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }

    // Remover usuário e notificar saída
    usuarios_clone2.write().unwrap().remove(&nome_usuario);
    let msg_saida = MensagemServidor::UsuarioSaiu {
        usuario: nome_usuario,
    };
    enviar_mensagem_todos(&usuarios_clone2, msg_saida);
}

// Função para enviar mensagem para todos os usuários conectados
fn enviar_mensagem_todos(usuarios: &Usuarios, mensagem: MensagemServidor) {
    // Criar lista de usuários para evitar deadlock
    let lista_usuarios: Vec<String> = {
        usuarios.read().unwrap().keys().cloned().collect()
    };

    // Enviar mensagem para cada usuário
    for usuario in lista_usuarios {
        if let Some(tx) = usuarios.read().unwrap().get(&usuario) {
            let _ = tx.send(mensagem.clone());
        }
    }
}