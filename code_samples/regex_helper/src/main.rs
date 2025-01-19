use std::collections::HashMap;
use regex::Regex;

fn gerar_regex(comando: &str) -> Result<String, String> {
    let mut mapeamento = HashMap::new();
    mapeamento.insert("uma letra", "[a-zA-Z]");
    mapeamento.insert("um dígito", "\\d");
    mapeamento.insert("qualquer caractere", ".");
    mapeamento.insert("opcional", "?");
    mapeamento.insert("início da string", "^");
    mapeamento.insert("fim da string", "$");
    mapeamento.insert("uma ou mais vezes", "+");
    mapeamento.insert("nenhuma ou mais vezes", "*");
    mapeamento.insert("um nome de usuário válido", "[a-zA-Z0-9._%+-]+");
    mapeamento.insert("um símbolo arroba", "@");
    mapeamento.insert("um domínio válido", "[a-zA-Z0-9.-]+");
    mapeamento.insert("uma extensão de domínio", "\\.[a-zA-Z]{2,}");

    let mut regex = String::new();
    let partes: Vec<&str> = comando.split("seguido de").map(|s| s.trim()).collect();

    for parte in partes {
        let mut encontrado = false;
        for (chave, valor) in &mapeamento {
            if parte.contains(chave) {
                regex.push_str(valor);
                encontrado = true;

                // Verifica por repetições
                if parte.contains("repetido de") {
                    if let Some(intervalo) = parte.split("repetido de").nth(1) {
                        let intervalo = intervalo.trim();
                        regex.push_str(&format!("{{{}}}", intervalo));
                    }
                }
                break;
            }
        }
        if !encontrado {
            return Err(format!("Comando não reconhecido: '{}'", parte));
        }
    }

    Ok(regex)
}

fn main() {
    // Comando para gerar a regex para validar emails
    let comando = "início da string seguido de um nome de usuário válido seguido de um símbolo arroba seguido de um domínio válido seguido de uma extensão de domínio seguido de fim da string";

    match gerar_regex(comando) {
        Ok(regex_str) => {
            println!("Regex gerada: {}", regex_str);

            // Compile a regex gerada
            match Regex::new(&regex_str) {
                Ok(regex) => {
                    // Testar a regex com emails de exemplo
                    let emails = vec![
                        "usuario@exemplo.com",
                        "teste.email@dominio.org",
                        "email_invalido@dominio",
                        "@semnome.com",
                        "nome@.com",
                        "email@dominio.com.br",
                    ];

                    for email in emails {
                        if regex.is_match(email) {
                            println!("'{}' é um email válido.", email);
                        } else {
                            println!("'{}' NÃO é um email válido.", email);
                        }
                    }
                }
                Err(err) => eprintln!("Erro ao compilar a regex: {}", err),
            }
        }
        Err(err) => eprintln!("Erro: {}", err),
    }
}
