struct Pessoa<'a> {
    nome: &'a str,
}

fn main() {
    let nome = String::from("Fulano");
    let pessoa = Pessoa { nome: &nome };

    println!("Nome da pessoa: {}", pessoa.nome);
}