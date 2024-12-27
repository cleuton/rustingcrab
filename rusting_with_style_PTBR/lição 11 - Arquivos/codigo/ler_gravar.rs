//! ```cargo
//! [package]
//! edition = "2021"
//! ```

use std::fs::File;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    // 1. Abra o arquivo "input.txt" para leitura
    let mut file = File::open("input.txt")?;
    
    // 2. Crie uma variável para armazenar o conteúdo lido
    let mut contents = String::new();
    
    // 3. Leia todo o conteúdo do arquivo para a variável 'contents'
    file.read_to_string(&mut contents)?;
    
    // 4. Exiba o conteúdo lido no terminal
    println!("Conteúdo do arquivo de entrada: {}", contents);
    
    // 5. Crie (ou sobrescreva) o arquivo "output.txt" para gravação
    let mut out_file = File::create("output.txt")?;
    
    // 6. Escreva algo no arquivo de saída; aqui, usamos o conteúdo que lemos
    writeln!(out_file, "Conteúdo copiado: {}", contents)?;
    
    // 7. Retorne Ok(()) caso tudo dê certo
    Ok(())
}