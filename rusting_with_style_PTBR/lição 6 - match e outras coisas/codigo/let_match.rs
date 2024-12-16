fn main() {
    // Tentamos converter uma string para um número inteiro
    let resultado = "42".parse::<i32>();

    let numero = match resultado {
        Ok(valor) => valor,                 // Se der certo, extraímos o valor
        Err(erro) => {                     // Se der errado, tratamos o erro
            println!("Erro ao converter: {}", erro);
            return;                        // Aqui poderíamos encerrar o programa ou tomar outra ação
        }
    };

    println!("Número convertido: {}", numero);
}

