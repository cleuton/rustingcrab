fn main() {

    // Option
    
    let numeros = vec![10, 20, 30];
    // Tentando acessar o índice 3, que não existe
    let valor = numeros.get(3);

    match valor {
        Some(v) => println!("O valor no índice 3 é {}", v),
        None => println!("Não existe valor nesse índice"),
    }

    // Result

    let resultado = "42".parse::<i32>(); // parse pode falhar se a string não for um número

    match resultado {
        Ok(numero) => println!("Número convertido com sucesso: {}", numero),
        Err(erro) => println!("Falha ao converter: {}", erro),
    }
}