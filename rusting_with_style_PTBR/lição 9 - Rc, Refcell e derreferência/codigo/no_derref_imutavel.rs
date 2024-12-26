fn main() {
    let x = 10;
    let x_ref = &x;

    // Acesso automático (mesmo sem usar `*`)
    println!("x_ref: {}", x_ref); // Imprime 10

    // Derreferência explícita
    println!("*x_ref: {}", *x_ref); // Imprime 10
}