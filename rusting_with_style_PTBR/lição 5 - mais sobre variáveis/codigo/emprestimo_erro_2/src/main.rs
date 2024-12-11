fn main() {
    let mut nome = String::from("João");
    let p = &nome; // Empréstimo imutável
    let q = &mut nome; // Erro! Não pode ter um empréstimo mutável enquanto há empréstimos imutáveis ativos
    
    println!("{}", p);
    println!("{}", q);
}