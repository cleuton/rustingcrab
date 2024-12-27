use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
	let mut arq = File::open("input.txt")?;
	let mut conteudo = String::new();
	arq.read_to_string(&mut conteudo)?;
	println!("Conteudo: {}", conteudo);
	Ok(())
}
