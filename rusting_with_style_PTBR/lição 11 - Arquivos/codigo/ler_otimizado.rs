//! ```cargo
//! [package]
//! edition = "2021"
//! ```

use std::fs::File;
use std::io::{self, BufReader, Read};

/// Lê todo o conteúdo de um objeto que implementa `Read` e retorna uma `String`.
fn read_all<R: Read>(mut reader: R) -> io::Result<String> {
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> io::Result<()> {
    // Exemplo com um arquivo
    let file = File::open("input.txt")?;
    let buf_reader = BufReader::new(file);

    let contents = read_all(buf_reader)?;
    println!("Conteúdo:\n{}", contents);

    Ok(())
}
