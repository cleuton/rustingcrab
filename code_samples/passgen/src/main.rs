mod modulos;
use std::env;
use std::io::{self};
use rand::seq::SliceRandom;
use rand::thread_rng;
use modulos::toml_proc::{Regra,ler_regra,salvar_senha};

pub fn gerar_senha(regra: &Regra) -> String {
    let mut rng = thread_rng();

    // Conjuntos de caracteres para cada regra
    let minusculas: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let maiusculas: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let digitos: Vec<char> = "0123456789".chars().collect();
    let especiais: Vec<char> = "!@#$%^&*()-_=+[]{}|;:,.<>?/".chars().collect();

    // Vetor para armazenar o pool de caracteres
    let mut pool: Vec<char> = Vec::new();

    // Adiciona ao pool de caracteres de acordo com as regras
    if regra.minusculas {
        pool.extend(&minusculas);
    }
    if regra.maiusculas {
        pool.extend(&maiusculas);
    }
    if regra.caracteres_especiais {
        pool.extend(&especiais);
    }
    if regra.pelo_menos_um_digito {
        pool.extend(&digitos);
    }

    // Vetor para armazenar a senha
    let mut senha_chars: Vec<char> = Vec::new();

    // Garante os requisitos obrigatórios
    if regra.pelo_menos_uma_letra_maiuscula {
        senha_chars.push(*maiusculas.choose(&mut rng).unwrap());
    }
    if regra.pelo_menos_um_caracter_especial {
        senha_chars.push(*especiais.choose(&mut rng).unwrap());
    }
    if regra.pelo_menos_um_digito {
        senha_chars.push(*digitos.choose(&mut rng).unwrap());
    }

    // Gera os caracteres restantes para completar o tamanho da senha
    let restante = (regra.tamanho_senha as usize).saturating_sub(senha_chars.len());
    for _ in 0..restante {
        senha_chars.push(*pool.choose(&mut rng).unwrap());
    }

    // Embaralha todos os caracteres da senha
    senha_chars.shuffle(&mut rng);

    senha_chars.into_iter().collect()
}

fn main() -> io::Result<()> {
    let regra = ler_regra()?;
    
    let senha = gerar_senha(&regra);

    // Lê os argumentos da linha de comando
    let args: Vec<String> = env::args().collect();

    // Verifica se um argumento foi passado
    if args.len() > 1 {
        let url = &args[1];
        salvar_senha(&url, &senha)?;
    } else {
        println!("Senha gerada: {}", senha);
    }

    Ok(())
}

