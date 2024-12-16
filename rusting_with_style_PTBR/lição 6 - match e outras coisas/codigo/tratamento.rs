use std::io;

fn ler_numero() -> Result<i32, std::num::ParseIntError> {
    let mut entrada = String::new();
    println!("Digite um número: ");

    io::stdin()
        .read_line(&mut entrada)
        .expect("Falha ao ler a entrada");

    let numero = entrada.trim().parse::<i32>();
    match numero {
        Ok(n) => Ok(n),
        Err(e) => Err(e),
    }
}

fn main() {
    match ler_numero() {
        Ok(num) => {
            println!("Você digitou o número: {}", num);
        }
        Err(_) => {
            println!("Não foi possível converter a entrada em um número inteiro.");
        }
    }
}
