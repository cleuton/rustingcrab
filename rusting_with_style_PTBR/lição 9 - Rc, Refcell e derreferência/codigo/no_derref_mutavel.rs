fn main() {
    let mut y = 20;
    let y_ref = &mut y;

    // Precisamos usar `*` para modificar o valor!!!!!
    *y_ref += 1;
    println!("y: {}", y); // Imprime 21
}