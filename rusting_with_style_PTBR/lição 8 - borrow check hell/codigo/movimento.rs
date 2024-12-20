fn main() {
    let s1 = String::from("Olá");
    let s2 = s1; // Movimento: s1 deixa de ser válido
    // println!("{}", s1); // Erro! s1 não é mais válido
    println!("{}", s2); // Funciona
}