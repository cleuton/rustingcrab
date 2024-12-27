//! ```cargo
//! [package]
//! edition = "2021"
//! [dependencies]
//! serde = { version = "1.0", features = ["derive"] }
//! serde_json = "1.0"
//! ```

use std::error::Error;
use std::fs::File;
use serde::Serialize;
use serde_json;

#[derive(Debug, Serialize)]
struct Pessoa {
    id: u32,
    nome: String,
    cidade: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Cria alguns dados de exemplo
    let lista_de_pessoas = vec![
        Pessoa {
            id: 1,
            nome: "Ana".to_string(),
            cidade: "São Paulo".to_string(),
        },
        Pessoa {
            id: 2,
            nome: "Bruno".to_string(),
            cidade: "Rio de Janeiro".to_string(),
        },
        Pessoa {
            id: 3,
            nome: "Carla".to_string(),
            cidade: "Belo Horizonte".to_string(),
        },
    ];

    // Cria (ou sobrescreve) o arquivo saida.json
    let arquivo = File::create("pessoas.json")?;

    // Escreve em formato JSON "bonito" (com indentação)
    serde_json::to_writer_pretty(arquivo, &lista_de_pessoas)?;

    println!("Arquivo JSON 'pessoas.json' gravado com sucesso!");

    Ok(())
}