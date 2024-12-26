fn main() {
    let x = 10;
    let y = 5;

    // Definindo uma closure de múltiplas linhas:

    let closure_multilinhas = |a: i32, b: i32| {
        let soma = a + b; 
        let produto = a * b; 
        soma + produto + x + y // Soma do resultado com as variáveis do escopo externo
    };

    let resultado = closure_multilinhas(3, 4); // Chamando a closure com os valores 3 e 4
    println!("O resultado é: {}", resultado); 
}