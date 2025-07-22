use axum::{
    routing::{get, post},
    Extension, Router,
};
use async_graphql::{
    Context, EmptySubscription, Object, Schema, SimpleObject,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use std::{net::SocketAddr, sync::{Arc, Mutex}};
use tower_http::cors::{Any, CorsLayer};

/// Objeto de domínio: usuário simples
#[derive(SimpleObject, Clone)]
struct Usuario {
    id: i32,
    nome: String,
}

/// Raiz das consultas (queries) GraphQL
struct RaizDeConsulta;

#[Object]
impl RaizDeConsulta {
    /// Lê todos os usuários do estado compartilhado
    async fn usuarios(&self, ctx: &Context<'_>) -> Vec<Usuario> {
        let db = ctx.data::<Arc<Mutex<Vec<Usuario>>>>().unwrap();
        let guard = db.lock().unwrap();
        guard.clone()
    }
}

/// Raiz das mutações (mutations) GraphQL
struct RaizDeMutacao;

#[Object]
impl RaizDeMutacao {
    /// Cria um novo usuário e adiciona ao vetor compartilhado
    async fn criar_usuario(&self, ctx: &Context<'_>, id: i32, nome: String) -> Usuario {
        let usuario = Usuario { id, nome: nome.clone() };
        let db = ctx.data::<Arc<Mutex<Vec<Usuario>>>>().unwrap();
        db.lock().unwrap().push(usuario.clone());
        usuario
    }
}

#[tokio::main]
async fn main() {
    // Estado inicial
    let estado_usuarios = Arc::new(Mutex::new(vec![
        Usuario { id: 1, nome: "Fulano".into() },
        Usuario { id: 2, nome: "Cicrano".into() },
    ]));

    // Monta o schema e injeta o estado como dado
    let esquema = Schema::build(
        RaizDeConsulta,
        RaizDeMutacao,
        EmptySubscription,
    )
    .data(estado_usuarios)
    .finish();

    // Constrói o servidor Axum
    let aplicacao = Router::new()
        .route("/graphql", post(manipulador_graphql))
        .route("/playground", get(abrir_playground))
        .layer(Extension(esquema))
        .layer(CorsLayer::new().allow_origin(Any));

    let endereco: SocketAddr = "0.0.0.0:4000".parse().unwrap();
    println!("Playground em http://{}", endereco);

    axum::Server::bind(&endereco)
        .serve(aplicacao.into_make_service())
        .await
        .unwrap();
}

/// Handler principal de operações GraphQL
async fn manipulador_graphql(
    Extension(esquema): Extension<Schema<RaizDeConsulta, RaizDeMutacao, EmptySubscription>>,
    requisicao: GraphQLRequest,
) -> GraphQLResponse {
    esquema.execute(requisicao.into_inner()).await.into()
}

/// Serve o GraphQL Playground (IDE web)
async fn abrir_playground() -> axum::response::Html<String> {
    axum::response::Html(
        async_graphql::http::playground_source(
            async_graphql::http::GraphQLPlaygroundConfig::new("/graphql"),
        )
    )
}
