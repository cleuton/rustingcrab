fn main() {
    let nome = String::from("João");
    let p = &nome; // p agora "observa" o valor de nome. É um "empréstimo" de nome para p.
    println!("{}", nome); // Válido
    println!("{}", p);    // Também válido
}