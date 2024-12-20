fn main() {
    let s = String::from("Olá");
    let r1 = &s; // Empréstimo imutável
    let r2 = &s; // Outro empréstimo imutável
    println!("{} e {}", r1, r2); // Funciona
  
    let mut s_mut = String::from("Olá");
    let r_mut = &mut s_mut; // Empréstimo mutável
    r_mut.push_str(" Mundo!");
    println!("{}", r_mut); // Funciona
}