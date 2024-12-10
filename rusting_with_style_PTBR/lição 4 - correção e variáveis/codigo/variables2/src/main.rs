fn main() {
    // Variáveis de tipos inteiros
    let inteiro: i32 = 42;
    let pequeno_inteiro: u8 = 255; // Unsigned 8-bit integer

    // Variáveis de ponto flutuante
    let flutuante: f64 = 3.1415;
    let flutuante_menor: f32 = 2.718;

    // Variável booleana
    let verdadeiro: bool = true;
    let falso: bool = false;

    // Variáveis de string
    let texto: &str = "Olá, mundo!"; // String slice
    let mut string: String = String::from("Rust é incrível!");

    // Exibindo os valores
    println!("Número inteiro: {}", inteiro);
    println!("Pequeno inteiro (u8): {}", pequeno_inteiro);
    println!("Número de ponto flutuante (f64): {}", flutuante);
    println!("Número de ponto flutuante menor (f32): {}", flutuante_menor);
    println!("Valor booleano verdadeiro: {}", verdadeiro);
    println!("Valor booleano falso: {}", falso);
    println!("Texto: {}", texto);
    println!("String mutável: {}", string);

    // Modificando a string mutável
    string.push_str(" Vamos aprender Rust!");
    println!("String após modificação: {}", string);
}
