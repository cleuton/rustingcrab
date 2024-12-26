//! ```cargo
//! [package]
//! edition = "2021"
//! ```

fn main() {
    // Uso de closures com funções de alta ordem
    let numeros = vec![1, 2, 3, 4, 5];
    let numeros_dobrados: Vec<i32> = numeros.iter().map(|&x| x * 2).collect();
    println!("Números dobrados: {:?}", numeros_dobrados);

    // Uso de closures com filter
    let numeros_pares: Vec<i32> = numeros.iter().filter(|x| *x % 2 == 0).map(|&x| x).collect();
    // Também funcionaria assim: 
    // let numeros_pares: Vec<i32> = numeros.iter().filter(|x| *x % 2 == 0).copied().collect();
    println!("Números pares: {:?}", numeros_pares);
}