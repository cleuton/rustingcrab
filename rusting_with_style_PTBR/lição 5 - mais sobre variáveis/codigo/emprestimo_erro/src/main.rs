fn main() {
    let nome = String::from("João");
    let p = &nome; // Empréstimo imutável
    
    // Tentar modificar `nome` enquanto `p` está emprestando
    nome.push_str(" Silva"); // Erro! Você não pode alterar `nome` enquanto `p` está referenciando.
    
    println!("{}", nome);
    println!("{}", p);
}
