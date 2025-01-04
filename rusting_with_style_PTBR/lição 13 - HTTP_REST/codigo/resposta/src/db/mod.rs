pub mod schema;
pub mod models;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::db::models::{Pessoa, NovaPessoa};
use crate::db::schema::pessoas::dsl::*;

/// Cria uma conexão com o PostgreSQL, lendo `DATABASE_URL` do .env
fn estabelecer_conexao() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL não definida no .env");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Falha ao conectar em {}", database_url))
}

/// Exemplo de inserir uma nova pessoa (caso queira usar em testes)
#[allow(dead_code)]
pub fn inserir_pessoa(nome_pessoa: &str) {
    let mut conn = estabelecer_conexao();
    let nova = NovaPessoa { nome: nome_pessoa };

    diesel::insert_into(pessoas)
        .values(&nova)
        .execute(&mut conn)
        .expect("Erro ao inserir pessoa");
}

/// Função que retorna a lista de pessoas do banco
pub fn listar_pessoas() -> Vec<Pessoa> {
    let mut conn = estabelecer_conexao();

    pessoas
        .load::<Pessoa>(&mut conn)
        .expect("Erro ao carregar pessoas")
}
