/// Este código dará erro "borrow of moved value: `nome`" porque a variável `nome` foi movida para `p` e não pode ser usada novamente.
fn main() {
    let nome = String::from("João");
    let p = nome;
    println!("{}", nome);
    println!("{}", p);
}
