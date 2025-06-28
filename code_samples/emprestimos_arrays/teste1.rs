
fn aloca(v: [u8;5])  {
    // "v" é uma cópia do array original.
    // Arrays de tamanho fixo implementam Copy, 
    // desde que seus elementos também implementem Copy.
    println!("Tamanho: {:?}", v.len());
}

fn mostra(v: [u8;5]) {
    // O mesmo se aplica aqui
    for i in v.iter() {
        println!("Valor: {}", i);
    }
}

fn main() {
    // Array mutável de tamanho fixo
    let mut a = [1,2,3,4,5];
    // "b" é uma referência imutável ao array "a".
    let b = &a;
    // Alteramos um dos elementos de "a"
    a[0] = 10; 
    // "aloca" recebe uma cópia do array "a"
    aloca(a);
    // "mostra" recebe uma cópia do array "a"
    mostra(a);
    // "b" ainda aponta para o array original, que foi modificado.
    // A linha abaixo causaria um erro de compilação, pois 
    // violaria a segurança de memória do Rust, já que "b"
    // é um "empréstimo" e a variável original foi alterada
    // enquanto "b" ainda está em uso.
    println!("Valor de b: {:?}", b);
}