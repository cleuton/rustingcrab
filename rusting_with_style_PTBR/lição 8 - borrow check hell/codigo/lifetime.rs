struct Pessoa<'a> {
    nome: &'a str,
}

fn main() {
    let nome = String::from("Alice");
    let pessoa = Pessoa { nome: &nome };
    println!("Nome: {}", pessoa.nome);
}