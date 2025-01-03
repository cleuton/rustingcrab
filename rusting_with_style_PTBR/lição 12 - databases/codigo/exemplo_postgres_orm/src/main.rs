mod schema;
mod models;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

// Importa as coisas que vamos usar
use crate::models::{NovaPessoa, Pessoa};
use crate::schema::pessoas::dsl::*;

fn estabelecer_conexao() -> PgConnection {
    dotenv().ok(); // Lê variáveis do arquivo .env
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL não definida no .env");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Falha ao conectar em {}", database_url))
}

fn main() {
    // 1. Cria a conexão mutável
    let mut conexao = estabelecer_conexao();

    // 2. Insere uma nova pessoa
    let maria = NovaPessoa { nome: "Maria das Couves" };

    diesel::insert_into(pessoas)
        .values(&maria)
        // repare que passamos &mut conexao
        .execute(&mut conexao)
        .expect("Erro ao inserir pessoa");

    // 3. Consulta a tabela
    let resultado = pessoas
        .load::<Pessoa>(&mut conexao) // &mut conexao
        .expect("Erro ao carregar pessoas");

    println!("Lista de pessoas:");
    for p in resultado {
        println!("ID: {}, Nome: {}", p.id, p.nome);
    }
}
