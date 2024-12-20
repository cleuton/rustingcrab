fn main() {
    let mut s = String::from("Olá");

    let r1 = &s; // Referência imutável.
    let r2 = &mut s; // Erro! Não pode ter referência mutável enquanto referências imutáveis existem.

    println!("{}, {}", r1, r2);
}