<img src="../guia_apis_rust.jpg" height=400>

---

<img src="../../../rusting-crab-logo.png" height=300>

---

# Guia das APIs em Rust

[**VER NO GITHUB**](https://github.com/cleuton/rustingcrab/tree/main/code_samples/guia_apis_rust/grpc).

## gRPC

<a href="https://github.com/cleuton/rustingcrab">Guia das APIs em Rust e todo código-fonte incluído</a> © 2025 by <a href="https://github.com/cleuton">Cleuton Sampaio</a> is licensed under <a href="https://creativecommons.org/licenses/by-sa/4.0/">CC BY-SA 4.0</a><img src="https://mirrors.creativecommons.org/presskit/icons/cc.svg" alt="" style="max-width: 1em;max-height:1em;margin-left: .2em;"><img src="https://mirrors.creativecommons.org/presskit/icons/by.svg" alt="" style="max-width: 1em;max-height:1em;margin-left: .2em;"><img src="https://mirrors.creativecommons.org/presskit/icons/sa.svg" alt="" style="max-width: 1em;max-height:1em;margin-left: .2em;">

Este é um **ebook** interativo, com código-fonte disponível para você utilizar à vontade. Leia a licença de uso. 

[**Retornar ao menu**](../)

**gRPC** é um framework de comunicação remota de alto desempenho desenvolvido pela **Google**, que permite que sistemas diferentes se comuniquem de forma rápida e eficiente, principalmente em ambientes distribuídos e microserviços. 

Ele usa o protocolo **HTTP/2** como transporte e **Protocol Buffers** (protobuf) como linguagem de definição de interface e serialização de dados, o que torna as mensagens menores, mais rápidas e mais estruturadas do que formatos como JSON. 

No gRPC, você define previamente os serviços e suas funções em um arquivo `.proto`, especificando as mensagens de entrada e saída, e o sistema gera automaticamente o código cliente e servidor em várias linguagens, garantindo consistência e reduzindo erros. 

Ele suporta quatro tipos de chamadas: chamada simples (unária), streaming do cliente, streaming do servidor e streaming bidirecional, sendo ideal para cenários que exigem baixa latência, como transmissão de dados em tempo real, sistemas de mensageria ou interações entre serviços internos.

Por ser mais eficiente que APIs baseadas em texto como **REST+JSON**, o gRPC reduz o uso de largura de banda, melhora a velocidade de comunicação e é amplamente usado em backends modernos, especialmente onde desempenho, escalabilidade e comunicação entre serviços são críticos.

## Nosso exemplo de API

Para começar, vejamos nosso arquivo `.proto` que define essa API: 

```proto
syntax = "proto3";
package tarefa;

// Serviço de gerenciamento de tarefas
service TarefaService {
  // Retorna todas as tarefas
  rpc Listar (ListaTarefasRequest) returns (ListaTarefasResponse);
  // Adiciona uma tarefa e devolve a criada
  rpc Criar (CriarTarefaRequest) returns (Tarefa);
}

message Tarefa {
  int32 id = 1;
  string descricao = 2;
  bool concluida = 3;
}

message ListaTarefasRequest {}

message ListaTarefasResponse {
  repeated Tarefa tarefas = 1;
}

message CriarTarefaRequest {
  string descricao = 1;
}
``` 

Este arquivo `.proto` define uma API simples para gerenciar tarefas usando gRPC. Ele especifica como os dados são estruturados e como os clientes podem interagir com o servidor.  

O `syntax = "proto3"` indica que está usando a terceira versão da linguagem Protocol Buffers.  
O `package tarefa;` organiza o código em um namespace chamado "tarefa", evitando conflitos de nomes.  

O `service TarefaService` define um serviço com duas operações:  
- `Listar`: recebe uma `ListaTarefasRequest` (vazia) e retorna uma `ListaTarefasResponse` contendo uma lista de tarefas.  
- `Criar`: recebe uma `CriarTarefaRequest` com uma descrição e retorna uma `Tarefa` completa, já com ID, descrição e status de conclusão.  

A mensagem `Tarefa` representa uma tarefa com três campos: `id` (número), `descricao` (texto) e `concluida` (verdadeiro/falso).  
`ListaTarefasRequest` é vazia, pois listar não precisa de parâmetros.  
`ListaTarefasResponse` contém uma lista (`repeated`) de tarefas.  
`CriarTarefaRequest` contém apenas a `descricao` da nova tarefa.  

Em resumo: este arquivo define um serviço onde um cliente pode pedir todas as tarefas ou criar uma nova, com estrutura clara, eficiente e pronta para gerar código em várias linguagens.

Como é um protocolo binário, você precisa de código especial para interagir com as mensagens, tanto para o **servidor** como para o **cliente**.

O crate `tonic` é uma implementação moderna, assíncrona e segura de gRPC em Rust, projetada para trabalhar com HTTP/2 e Protocol Buffers (protobuf). Ele permite criar servidores e clientes gRPC com alto desempenho, aproveitando o sistema de tipos e a segurança do Rust. O `tonic` se integra diretamente com arquivos `.proto` para gerar código Rust automaticamente com base na definição de serviços e mensagens.

Para usar **gRPC** precisamos de geração de código especial para isso, e temos um script `build.rs` que será invocado pelo `Cargo` antes de compilar nosso código: 

```rust
fn main() {
    tonic_build::compile_protos("proto/tarefa.proto")
        .expect("Falha ao compilar proto/tarefa.proto");
}
``` 

O arquivo `build.rs` mostrado é um script de compilação personalizado do Cargo (sistema de build do Rust) que roda antes da compilação principal. Nele, `tonic_build::compile_protos("proto/tarefa.proto")` lê o arquivo `.proto`, processa suas definições de serviço, mensagens e tipos, e gera código Rust correspondente — como structs para as mensagens (`Tarefa`, `ListaTarefasRequest`, etc.) e traits para o serviço (`TarefaService`). Esse código gerado inclui:

- Estruturas Rust para cada `message` definida no `.proto`.
- Traits com métodos assíncronos para o servidor (ex: `async fn listar(&self, request: Request<ListaTarefasRequest>) -> Result<Response<ListaTarefasResponse>, Status>`).
- Implementações de serialização/desserialização usando `prost` (um gerador de código protobuf para Rust).
- Suporte a streaming e metadados.

Após a execução do `build.rs`, o código gerado é colocado automaticamente no diretório `OUT_DIR` e pode ser importado no seu projeto com `include!()`. Assim, você implementa o serviço apenas preenchendo os métodos da trait gerada, e o `tonic` cuida do resto: recebimento de requisições, parsing de dados, chamadas assíncronas e envio de respostas. Isso torna o desenvolvimento de APIs gRPC em Rust altamente produtivo, seguro e alinhado com as melhores práticas de desempenho e concorrência.

## O servidor

O código do servidor fica bem simplificado com o uso do `tonic`: 

```rust
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
``` 

Este código é um servidor gRPC em Rust que implementa um sistema simples de gerenciamento de tarefas, usando o arquivo `.proto`, definido anteriormente.

Vejamos cada parte desse código...

O bloco `mod tarefa { tonic::include_proto!("tarefa"); }` diz ao Rust para incluir o código gerado pelo `tonic` com base no arquivo `tarefa.proto`. Esse código contém as estruturas (`Tarefa`, `CriarTarefaRequest`, etc.) e o serviço gRPC (`TarefaService`). Tudo gerado com aquele `build.rs` que o `Cargo` vai executar automaticamente antes da compilação do servidor.

Depois, usamos essas estruturas e o serviço com `use`.

A estrutura `ServidorTarefa` representa o servidor real. Ela contém uma lista de tarefas protegida por um `Mutex` (para acesso seguro em múltiplas threads) e envolvida em um `Arc` (para compartilhar a mesma lista entre threads). Isso é para simular um `database`, mas você poderia usar `SQLX` aqui e `PostgreSQL`, por exemplo. 

`#[derive(Default)]` permite que o servidor comece com uma lista vazia.

O bloco `impl TarefaService for ServidorTarefa` implementa as duas funções definidas no `.proto`: `listar` e `criar`.

- `listar`: pega um *request* vazio, bloqueia o acesso à lista de tarefas, faz uma cópia e retorna uma resposta com todas as tarefas.
- `criar`: recebe um *request* com uma descrição, gera um novo ID, cria uma nova tarefa (não concluída), adiciona na lista e retorna a tarefa criada.

Ambas são `async` porque gRPC no `tonic` é assíncrono.

No `main`, usamos `#[tokio::main]` para rodar um runtime assíncrono (necessário para gRPC). Definimos o endereço do servidor (`[::1]:50051`, que é localhost na porta 50051).

Criamos uma instância do `ServidorTarefa` e iniciamos o servidor gRPC com `Server::builder()`, adicionamos o serviço e chamamos `.serve()` para escutar conexões.

## O cliente

Para criar clientes em qualquer linguagem (que suporte gRPC), você precisa apenas do arquivo `.proto`. Criei uma pasta `proto`, dentro da pasta `python_client`, e copiei o arquivo `.proto` para lá. Depois é preciso gerar os `stubs` para que o cliente **Python** possa usar nossa API.

Você precisa rodar esse comando porque ele **gera automaticamente o código Python necessário para o cliente (ou servidor) se comunicar via gRPC com base no seu arquivo `.proto`**.

Para isso precisamos rodar o `pip install -r requirements.txt` e instalar as dependências do `gRPC` para **Python**, e depois gerar o código a partir do `.proto`: 

```bash
python -m grpc_tools.protoc \
  -I proto \
  --python_out=. \
  --grpc_python_out=. \
  proto/tarefa.proto
```

### O que esse comando faz:

- `python -m grpc_tools.protoc`  
  → Executa a ferramenta `protoc` (compilador de Protocol Buffers) usando o plugin do gRPC para Python.

- `-I proto`  
  → Indica que o diretório `proto` contém os arquivos `.proto` que podem ser importados. É o "caminho de inclusão".

- `--python_out=.`  
  → Gera o código das **mensagens** (como `Tarefa`, `CriarTarefaRequest`, etc.) em Python e salva na pasta atual (`.`).

- `--grpc_python_out=.`  
  → Gera o código específico para **gRPC**:  
    - O cliente pode chamar `Criar()` e `Listar()`  
    - O servidor pode implementar essas funções  
  → Também salva na pasta atual.

- `proto/tarefa.proto`  
  → É o arquivo de definição da API que será processado.

### Resultado: dois arquivos são gerados

Depois de rodar o comando, você verá:

1. `tarefa_pb2.py`  
   → Contém as classes das **mensagens** (como `Tarefa`, `ListaTarefasRequest`, etc.).  
   → Gerado por `--python_out=.`  
   → `pb2` = "Protocol Buffers, versão 2" (convenção histórica).

2. `tarefa_pb2_grpc.py`  
   → Contém:
   - A classe `TarefaServiceStub` → para o **cliente** chamar os métodos (`Listar`, `Criar`)
   - A classe `TarefaServiceServicer` → para o **servidor** implementar os métodos

### Por que isso é necessário?

O Python não sabe, por si só, o que é `TarefaService`, `Listar` ou `Tarefa`.  
O arquivo `.proto` é só uma definição, e o código Python precisa ser **gerado** a partir dele.

### Finalmente, o código cliente

Aqui está o código cliente em **Python** que usa nossa API: 

```python
import grpc
import tarefa_pb2
import tarefa_pb2_grpc

def executar():
    with grpc.insecure_channel('localhost:50051') as canal:
        stub = tarefa_pb2_grpc.TarefaServiceStub(canal)
        # Lista as tarefas iniciais
        resposta = stub.Listar(tarefa_pb2.ListaTarefasRequest())
        print("Tarefas atuais:", [tarefa.descricao for tarefa in resposta.tarefas])

        # Cria uma nova tarefa
        nova = stub.Criar(tarefa_pb2.CriarTarefaRequest(descricao='Escrever relatório'))
        print("Tarefa criada:", nova.id, nova.descricao, "concluída?", nova.concluida)

        # Lista de novo para ver a inclusão
        resposta_atual = stub.Listar(tarefa_pb2.ListaTarefasRequest())
        print("Tarefas agora:", [t.descricao for t in resposta_atual.tarefas])

if __name__ == '__main__':
    executar()
```

Este código é um cliente Python que se comunica com um servidor gRPC escrito em Rust (ou qualquer outro lugar) usando o contrato definido no arquivo `tarefa.proto`. Vamos explicar de forma simples e objetiva:

O código começa importando três módulos:  
- `grpc`: a biblioteca do gRPC para Python, responsável pela comunicação de rede.  
- `tarefa_pb2`: contém as classes das mensagens geradas a partir do `.proto` (como `ListaTarefasRequest`, `Tarefa`, etc).  
- `tarefa_pb2_grpc`: contém o stub (cliente) que permite chamar os métodos remotos do serviço `TarefaService`.

A função `executar()` faz o seguinte:

1. **Conecta ao servidor gRPC**  
   Usa `grpc.insecure_channel('localhost:50051')` para abrir uma conexão não criptografada com o servidor rodando na porta 50051 do seu computador.

2. **Cria um stub (cliente) do serviço**  
   `TarefaServiceStub(canal)` é o objeto que você usa para chamar os métodos remotos (`Listar`, `Criar`) como se fossem funções locais.

3. **Chama o método `Listar`**  
   Envia uma requisição vazia (`ListaTarefasRequest()`) e recebe uma resposta com a lista de tarefas.  
   O código extrai e imprime as descrições das tarefas existentes.

4. **Chama o método `Criar`**  
   Envia uma requisição com a descrição "Escrever relatório".  
   O servidor responde com a tarefa criada (com ID gerado, descrição e `concluida=False`).  
   O cliente imprime os dados dessa nova tarefa.

5. **Chama `Listar` novamente**  
   Mostra a lista atualizada, agora incluindo a nova tarefa, para confirmar que a mudança foi persistida.

Por fim, `if __name__ == '__main__':` garante que a função `executar()` seja chamada apenas quando o script for executado diretamente.

## Executando a bagaça toda

É simples: 

1) Rode `cargo run` na [**pasta do projeto Rust**](./grpc_tarefas/). 
2) Rode `pip install -r requirements.txt` [**na pasta do projeto Python**](./python_client/).
3) Rode o comando `protoc` [**na pasta do projeto Python**](./python_client/): 

```bash
python -m grpc_tools.protoc \
  -I proto \
  --python_out=. \
  --grpc_python_out=. \
  proto/tarefa.proto
```

4) Execute o cliente: `python cliente.py`.


[**Retornar ao menu**](../)

