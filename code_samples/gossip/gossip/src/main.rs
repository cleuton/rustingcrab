use clap::Parser;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::sync::Mutex;
use tokio::io::AsyncBufReadExt;
use uuid::Uuid;

// --- Estruturas de Dados ---

/// Representa o estado de um nó.
/// Usamos um HashMap para armazenar dados (chave-valor) e suas versões.
/// A versão é um simples contador que incrementa a cada atualização.
type NodeState = HashMap<String, (String, u64)>;

/// Estrutura do nosso nó de gossip.
#[derive(Debug)]
struct Node {
    /// Endereço de escuta do nó (IP:PORTA).
    listen_addr: SocketAddr,
    /// ID único para este nó, para evitar se conectar a si mesmo.
    id: Uuid,
    /// A lista de outros nós conhecidos na rede.
    peers: Arc<Mutex<Vec<SocketAddr>>>,
    /// O estado (dados) deste nó.
    state: Arc<Mutex<NodeState>>,
}

/// A mensagem que será trocada entre os nós.
/// Ela contém o estado completo do nó remetente e sua lista de peers.
#[derive(Serialize, Deserialize, Debug)]
struct GossipMessage {
    node_id: Uuid,
    state: NodeState,
    peers: Vec<SocketAddr>,
}

// --- Lógica Principal ---

impl Node {
    /// Cria uma nova instância do nó.
    fn new(listen_addr: SocketAddr) -> Self {
        Node {
            listen_addr,
            id: Uuid::new_v4(),
            peers: Arc::new(Mutex::new(Vec::new())),
            state: Arc::new(Mutex::new(NodeState::new())),
        }
    }

    /// Inicia os processos principais do nó: escuta, fofoca e entrada do usuário.
    async fn start(&self, initial_peers: Vec<SocketAddr>) -> Result<(), Box<dyn std::error::Error>> {
        println!("[INFO] Nó ID: {}", self.id);
        println!("[INFO] Escutando em: {}", self.listen_addr);

        // Adiciona os peers iniciais informados via linha de comando.
        {
            let mut peers = self.peers.lock().await;
            for peer_addr in initial_peers {
                if !peers.contains(&peer_addr) {
                    peers.push(peer_addr);
                }
            }
            println!("[INFO] Peers iniciais: {:?}", peers);
        }

        let socket = Arc::new(UdpSocket::bind(self.listen_addr).await?);

        // Clona Arcs para mover para as tasks assíncronas
        let listen_socket = socket.clone();
        let listen_peers = self.peers.clone();
        let listen_state = self.state.clone();
        let listen_node_id = self.id;

        // --- Task 1: Escutar por mensagens de gossip ---
        tokio::spawn(async move {
            let mut buf = [0; 4096]; // Buffer para receber dados
            loop {
                match listen_socket.recv_from(&mut buf).await {
                    Ok((len, src_addr)) => {
                        // Deserializa a mensagem recebida
                        if let Ok(message) = serde_json::from_slice::<GossipMessage>(&buf[..len]) {
                            // Ignora mensagens de si mesmo
                            if message.node_id == listen_node_id {
                                continue;
                            }

                            println!("[GOSSIP] Mensagem recebida de {}", src_addr);
                            
                            // Processa a mensagem fundindo os estados e listas de peers
                            Self::merge_state_and_peers(
                                listen_state.clone(),
                                listen_peers.clone(),
                                message,
                                src_addr,
                            ).await;
                        }
                    }
                    Err(e) => eprintln!("[ERRO] Falha ao receber dados: {}", e),
                }
            }
        });

        // --- Task 2: Enviar "fofocas" periodicamente ---
        let gossip_socket = socket.clone();
        let gossip_peers = self.peers.clone();
        let gossip_state = self.state.clone();
        let gossip_node_id = self.id;

        tokio::spawn(async move {
            loop {
                // Intervalo da "fofoca"
                tokio::time::sleep(Duration::from_secs(2)).await;

                // Seleciona um peer aleatório para enviar a mensagem
                let target_peer = {
                    let peers = gossip_peers.lock().await;
                    peers.choose(&mut rand::thread_rng()).cloned()
                };

                if let Some(target) = target_peer {
                    let peers = gossip_peers.lock().await;
                    let state = gossip_state.lock().await;

                    // Cria a mensagem de gossip com o estado atual e a lista de peers
                    let message = GossipMessage {
                        node_id: gossip_node_id,
                        state: state.clone(),
                        peers: peers.clone(),
                    };

                    // Serializa e envia a mensagem via UDP
                    if let Ok(encoded) = serde_json::to_vec(&message) {
                        if let Err(e) = gossip_socket.send_to(&encoded, target).await {
                            eprintln!("[ERRO] Falha ao enviar gossip para {}: {}", target, e);
                        } else {
                            println!("[GOSSIP] Fofoca enviada para {}", target);
                        }
                    }
                }
            }
        });

        // --- Task 3: Lidar com a entrada do usuário para alterar o estado ---
        let state_handle = self.state.clone();
        tokio::spawn(async move {
            let mut stdin = tokio::io::BufReader::new(tokio::io::stdin());
            let mut line = String::new();
            loop {
                line.clear();
                // Aguarda a entrada do usuário de forma assíncrona
                if let Err(e) = stdin.read_line(&mut line).await {
                    eprintln!("[ERRO] Falha ao ler a entrada do terminal: {}", e);
                    break;
                }

                let parts: Vec<&str> = line.trim().splitn(3, ' ').collect();

                if parts.get(0) == Some(&"set") && parts.len() == 3 {
                    let key = parts[1].to_string();
                    let value = parts[2].to_string();

                    let mut state = state_handle.lock().await;

                    // Atualiza a versão do dado
                    let version = state.get(&key).map_or(1, |(_, v)| v + 1);
                    state.insert(key.clone(), (value.clone(), version));

                    println!("[ESTADO] Atualizado: {} = '{}' (versão {})", key, value, version);
                    Self::print_state(&state).await;

                } else {
                    println!("[CMD] Comando inválido. Use: set <chave> <valor>");
                }
            }
        });        
        
        // --- Task 4: Imprimir o estado atual periodicamente ---
        let print_state_handle = self.state.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(10)).await;
                let state = print_state_handle.lock().await;
                println!("\n--- Estado Atual do Nó ---");
                Self::print_state(&state).await;
                println!("------------------------\n");
            }
        });

        // Mantém a task principal viva
        std::future::pending::<()>().await;

        Ok(())
    }
    
    /// Imprime o estado atual do nó de forma legível.
    async fn print_state(state: &NodeState) {
        if state.is_empty() {
            println!("O estado está vazio.");
        } else {
            for (key, (value, version)) in state.iter() {
                println!("  - '{}': '{}' (v{})", key, value, version);
            }
        }
    }

    /// A lógica central do gossip: funde o estado recebido com o estado local.
    async fn merge_state_and_peers(
        local_state_arc: Arc<Mutex<NodeState>>,
        local_peers_arc: Arc<Mutex<Vec<SocketAddr>>>,
        remote_message: GossipMessage,
        remote_addr: SocketAddr,
    ) {
        let mut local_state = local_state_arc.lock().await;
        let mut state_changed = false;

        // 1. Fundir o estado (dados chave-valor)
        for (key, (remote_value, remote_version)) in remote_message.state {
            let local_entry = local_state.entry(key.clone()).or_insert((String::new(), 0));

            // A informação remota é mais nova, então atualizamos nosso estado local.
            if remote_version > local_entry.1 {
                *local_entry = (remote_value, remote_version);
                state_changed = true;
            }
        }
        
        if state_changed {
            println!("[MERGE] Estado local foi atualizado pela fofoca.");
            Self::print_state(&local_state).await;
        }

        // 2. Fundir a lista de peers
        let mut local_peers = local_peers_arc.lock().await;
        
        // Adiciona o remetente à nossa lista de peers, se ainda não o conhecemos.
        if !local_peers.contains(&remote_addr) {
            local_peers.push(remote_addr);
            println!("[PEERS] Novo peer descoberto: {}", remote_addr);
        }

        // Adiciona os peers do remetente à nossa lista, se ainda não os conhecemos.
        for peer in remote_message.peers {
            if peer != local_peers[0] && !local_peers.contains(&peer) {
                local_peers.push(peer);
                println!("[PEERS] Peer descoberto via fofoca: {}", peer);
            }
        }
    }
}

/// Define os argumentos da linha de comando.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// A porta em que este nó vai escutar.
    #[arg(long)]
    port: u16,

    /// O endereço de um peer existente para se conectar inicialmente.
    #[arg(long)]
    peer: Option<SocketAddr>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let listen_addr = format!("127.0.0.1:{}", args.port).parse()?;
    let node = Node::new(listen_addr);

    let initial_peers = args.peer.map_or(vec![], |p| vec![p]);

    node.start(initial_peers).await
}
