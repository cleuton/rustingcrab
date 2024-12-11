fn main() {
    let nome = String::from("João");
    let p = &nome; // Empréstimo imutável
    
    let outro = nome; // Erro! Você está tentando mover `nome` enquanto `p` ainda está emprestando
    
    println!("{}", p);
    println!("{}", outro);
}