#[derive(Debug)] // Implementa `Debug`, `Clone` e `Copy` para `Ponto`
struct Ponto {
    x: i32,
    y: i32,
}

fn main() {
    let ponto1 = Ponto { x: 10, y: 20 };
    
    // Usando `Copy`
    let ponto2 = ponto1; // `ponto1` ainda é válido porque `Ponto` implementa `Copy`
    
    println!("ponto1: {:?}", ponto1);
    println!("ponto2: {:?}", ponto2);
    
}