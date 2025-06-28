
fn aloca(v: &[String])  {
    println!("Tamanho: {:?}", v.len());
}

fn mostra(v: &[String]) {
    // O mesmo se aplica aqui
    for i in v.iter() {
        println!("Valor: {}", i);
    }
}

fn main() {
    // Array mutável de tamanho fixo que não implementa o trait Copy
    let mut a: [String; 5] = ["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(), "e".to_string()];
    // "b" é uma referência imutável ao array "a".
    let b = &a;
    // Ao descomentar a última linha (o println!) daria erro pois
    // o array "a" estaria "emprestado" para a variável "b"
    a[0] = "z".to_string(); 
    // O array "a" é movido para a função "aloca"
    aloca(&a);
    // A linha seguinte daria erro, pois o array que estava
    // em "a" foi movido para a função "aloca", e não pode ser usado novamente.
    mostra(&a);

    // "b" ainda aponta para o array original, que foi modificado.
    // A linha abaixo causaria um erro de compilação, pois 
    // violaria a segurança de memória do Rust, já que "b"
    // é um "empréstimo" e a variável original foi alterada
    // enquanto "b" ainda está em uso.
    println!("Valor de b: {:?}", b);
}