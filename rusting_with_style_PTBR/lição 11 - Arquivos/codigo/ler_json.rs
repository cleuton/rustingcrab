//! ```cargo
//! [package]
//! edition = "2021"
//! [dependencies]
//! serde = { version = "1.0", features = ["derive"] }
//! serde_json = "1.0"
//! ```

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use serde::Deserialize;
use serde_json;

#[derive(Debug, Deserialize)]
struct Pessoa {
    id: u32,
    nome: String,
    cidade: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Abre o arquivo
    let arquivo = File::open("pessoas.json")?;
    // Cria um BufReader para ler de forma eficiente
    let leitor = BufReader::new(arquivo);

    // Desserializa o conteúdo do JSON para um vetor de Pessoa
    let lista_de_pessoas: Vec<Pessoa> = serde_json::from_reader(leitor)?;

    // Exibe as pessoas lidas
    for pessoa in lista_de_pessoas {
        println!("{:?}", pessoa);
    }

    Ok(())
}