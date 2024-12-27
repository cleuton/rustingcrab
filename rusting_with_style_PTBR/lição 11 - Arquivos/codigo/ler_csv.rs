//! ```cargo
//! [package]
//! edition = "2021"
//! [dependencies]
//! csv = "1"
//! serde = { version = "1.0", features = ["derive"] }
//! serde_derive = "1.0"
//! ```

use std::error::Error;
use serde::Deserialize;
use csv::ReaderBuilder;

#[derive(Debug, Deserialize)]
struct Pessoa {
    id: u32,
    nome: String,
    cidade: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Cria um reader (leitor) para abrir e ler o arquivo CSV
    let mut leitor = ReaderBuilder::new()
        .has_headers(true)   // Indica que a primeira linha do CSV são cabeçalhos
        .from_path("dados.csv")?;

    // Lê cada registro (linha) do arquivo, desserializando em uma struct Pessoa
    for resultado in leitor.deserialize::<Pessoa>() {
        // Se ocorrer algum erro, o ? propaga o erro automaticamente
        let registro = resultado?;
        
        // Mostra o registro lido
        println!("{:?}", registro);
    }

    Ok(())
}
