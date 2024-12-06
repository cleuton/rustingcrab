/*
Implementação do algoritmo Shunting Yard © 2024 por Cleuton Sampaio
licenciado sob CC BY-SA 4.0. Para ver uma cópia desta licença,
visite https://creativecommons.org/licenses/by-sa/4.0/
*/

use std::f64::consts::PI;

// Verifica se um caractere é operador
fn e_operador(c: char) -> bool {
    c == '+' || c == '-' || c == '*' || c == '/' || c == '^'
}

// Retorna a precedência do operador
fn precedencia(op: char) -> i32 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        '^' => 3,
        _ => 0,
    }
}

// Verifica se uma string é uma função válida
fn e_funcao(token: &str) -> bool {
    token == "EXP" || token == "SQR" || token == "SIN" || token == "COS"
}

// Analisa a expressão e divide em tokens
fn tokenizar(infixa: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut token = String::new();
    let mut esperando_operando = true; // Indica se estamos esperando um operando (usado para operadores unários)

    let chars: Vec<char> = infixa.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i].is_whitespace() {
            i += 1;
            continue;
        }
        if chars[i].is_digit(10) || chars[i] == '.' {
            token.push(chars[i]);
            while i + 1 < chars.len() && (chars[i + 1].is_digit(10) || chars[i + 1] == '.') {
                i += 1;
                token.push(chars[i]);
            }
            tokens.push(token.clone());
            token.clear();
            esperando_operando = false;
        } else if chars[i].is_alphabetic() {
            token.push(chars[i]);
            while i + 1 < chars.len() && chars[i + 1].is_alphabetic() {
                i += 1;
                token.push(chars[i]);
            }
            tokens.push(token.clone());
            token.clear();
            esperando_operando = false;
        } else if e_operador(chars[i]) {
            if esperando_operando && chars[i] == '-' {
                // Trata o operador unário
                token.push('-');
                if i + 1 < chars.len() && (chars[i + 1].is_digit(10) || chars[i + 1] == '.') {
                    i += 1;
                    token.push(chars[i]);
                    while i + 1 < chars.len() && (chars[i + 1].is_digit(10) || chars[i + 1] == '.') {
                        i += 1;
                        token.push(chars[i]);
                    }
                }
                tokens.push(token.clone());
                token.clear();
                esperando_operando = false;
            } else {
                tokens.push(chars[i].to_string());
                esperando_operando = true;
            }
        } else if chars[i] == '(' || chars[i] == ')' {
            tokens.push(chars[i].to_string());
            esperando_operando = chars[i] == '(';
        } else {
            // Caractere inválido
            tokens.push(chars[i].to_string());
            esperando_operando = true;
        }
        i += 1;
    }
    tokens
}

// Verifica se uma expressão infixa é válida
fn validar_expressao(tokens: &[String]) -> &str {
    let mut balanceamento_parenteses = 0;
    for i in 0..tokens.len() {
        let token = &tokens[i];
        if token == "(" {
            balanceamento_parenteses += 1;
        } else if token == ")" {
            balanceamento_parenteses -= 1;
            if balanceamento_parenteses < 0 {
                return "Parenteses desbalanceados";
            }
        } else if e_operador(token.chars().next().unwrap()) && token.len() == 1 {
            if i == 0 || i == tokens.len() - 1 {
                return "Expressao invalida"; // Operador no início ou no fim
            }
            if i + 1 < tokens.len()
                && e_operador(tokens[i + 1].chars().next().unwrap())
                && tokens[i + 1].len() == 1
            {
                return "Expressao invalida"; // Operadores duplos
            }
        } else if e_funcao(token) {
            if i + 1 >= tokens.len() || tokens[i + 1] != "(" {
                return "Expressao invalida"; // Função deve ser seguida por '('
            }
        } else if token.chars().next().unwrap().is_digit(10)
            || (token.len() > 1 && token.chars().nth(1).unwrap().is_digit(10))
        {
            // Número é considerado válido
        } else {
            return "Expressao invalida";
        }
    }
    if balanceamento_parenteses == 0 {
        "OK"
    } else {
        "Parenteses desbalanceados"
    }
}

// Converte uma expressão infixa para posfixa usando o algoritmo de Shunting Yard
fn infixa_para_posfixa(infixa: &str) -> String {
    let mut operadores: Vec<String> = Vec::new();
    let mut saida = String::new();
    let tokens = tokenizar(infixa);

    let resultado_validacao = validar_expressao(&tokens);
    if resultado_validacao != "OK" {
        return resultado_validacao.to_string();
    }

    for token in tokens {
        if token.chars().next().unwrap().is_digit(10)
            || (token.len() > 1 && token.chars().nth(1).unwrap().is_digit(10))
        {
            // Token é um operando (número)
            saida.push_str(&token);
            saida.push(' ');
        } else if e_funcao(&token) {
            // Token é uma função
            operadores.push(token);
        } else if token == "(" {
            // Token é um parêntese de abertura
            operadores.push(token);
        } else if token == ")" {
            // Token é um parêntese de fechamento
            while !operadores.is_empty() && operadores.last().unwrap() != "(" {
                saida.push_str(&operadores.pop().unwrap());
                saida.push(' ');
            }
            if !operadores.is_empty() && operadores.last().unwrap() == "(" {
                operadores.pop(); // Remove o '('
            } else {
                return "Parenteses desbalanceados".to_string();
            }
            if !operadores.is_empty() && e_funcao(operadores.last().unwrap()) {
                saida.push_str(&operadores.pop().unwrap());
                saida.push(' ');
            }
        } else if e_operador(token.chars().next().unwrap()) {
            while !operadores.is_empty()
                && ((token.chars().next().unwrap() != '^'
                    && precedencia(operadores.last().unwrap().chars().next().unwrap())
                        >= precedencia(token.chars().next().unwrap()))
                    || (token.chars().next().unwrap() == '^'
                        && precedencia(operadores.last().unwrap().chars().next().unwrap())
                            > precedencia(token.chars().next().unwrap())))
            {
                saida.push_str(&operadores.pop().unwrap());
                saida.push(' ');
            }
            operadores.push(token);
        }
    }

    // Esvazia a pilha de operadores
    while !operadores.is_empty() {
        if operadores.last().unwrap() == "(" {
            return "Parenteses desbalanceados".to_string();
        }
        saida.push_str(&operadores.pop().unwrap());
        saida.push(' ');
    }

    saida
}

// Calcula o seno de um ângulo em graus
fn seno_graus(valor: f64) -> f64 {
    (valor * PI / 180.0).sin()
}

// Calcula o cosseno de um ângulo em graus
fn cosseno_graus(valor: f64) -> f64 {
    (valor * PI / 180.0).cos()
}

// Avalia uma expressão em notação polonesa reversa (RPN)
fn avaliar_rpn(rpn: &str) -> Result<f64, String> {
    let mut pilha: Vec<f64> = Vec::new();
    let tokens = rpn.split_whitespace();

    for token in tokens {
        if token.chars().next().unwrap().is_digit(10)
            || (token.len() > 1 && token.chars().nth(1).unwrap().is_digit(10))
        {
            pilha.push(token.parse::<f64>().map_err(|_| "Número inválido")?);
        } else if e_operador(token.chars().next().unwrap()) {
            if pilha.len() < 2 {
                return Err("Expressão RPN inválida".to_string());
            }
            let b = pilha.pop().unwrap();
            let a = pilha.pop().unwrap();
            match token.chars().next().unwrap() {
                '+' => pilha.push(a + b),
                '-' => pilha.push(a - b),
                '*' => pilha.push(a * b),
                '/' => pilha.push(a / b),
                '^' => pilha.push(a.powf(b)),
                _ => return Err("Operador desconhecido".to_string()),
            }
        } else if e_funcao(token) {
            if pilha.is_empty() {
                return Err("Expressão RPN inválida".to_string());
            }
            let a = pilha.pop().unwrap();
            if token == "SIN" {
                pilha.push(seno_graus(a));
            } else if token == "COS" {
                pilha.push(cosseno_graus(a));
            } else if token == "EXP" {
                pilha.push(a.exp());
            } else if token == "SQR" {
                pilha.push(a.sqrt());
            } else {
                return Err("Função desconhecida".to_string());
            }
        } else {
            return Err("Token desconhecido".to_string());
        }
    }

    if pilha.len() != 1 {
        return Err("Expressão RPN inválida".to_string());
    }
    Ok(pilha.pop().unwrap())
}

// Função de teste
fn executar_testes() {
    struct TestCase {
        infixa: &'static str,
        rpn_esperada: &'static str,
        valor_esperado: f64,
    }

    let casos_de_teste = vec![
        TestCase {
            infixa: "3+4*2/(1-5)^2^3",
            rpn_esperada: "3 4 2 * 1 5 - 2 3 ^ ^ / + ",
            valor_esperado: 3.0001220703125,
        },
        TestCase {
            infixa: "SIN(3+4)*COS(2-1)",
            rpn_esperada: "3 4 + SIN 2 1 - COS * ",
            valor_esperado: 0.121851,
        },
        TestCase {
            infixa: "-3+4*-2/(1--5)^2^3",
            rpn_esperada: "-3 4 -2 * 1 -5 - 2 3 ^ ^ / + ",
            valor_esperado: -3.0001220703125,
        },
        TestCase {
            infixa: "3++4",
            rpn_esperada: "Expressao invalida",
            valor_esperado: 0.0,
        },
        TestCase {
            infixa: "SIN(3+4)*INVALID(2-1)",
            rpn_esperada: "Expressao invalida",
            valor_esperado: 0.0,
        },
        TestCase {
            infixa: "3+4**2",
            rpn_esperada: "Expressao invalida",
            valor_esperado: 0.0,
        },
        TestCase {
            infixa: "(3+4",
            rpn_esperada: "Parenteses desbalanceados",
            valor_esperado: 0.0,
        },
    ];

    for teste in casos_de_teste {
        let rpn_resultante = infixa_para_posfixa(teste.infixa);
        println!("Infixa: {}", teste.infixa);
        println!("RPN esperada: {}", teste.rpn_esperada);
        println!("RPN retornada: {}", rpn_resultante);

        if rpn_resultante == teste.rpn_esperada {
            println!("Conversao em RPN: OK");
        } else {
            println!("Conversao em RPN: FALHA");
        }

        if rpn_resultante == teste.rpn_esperada
            && rpn_resultante != "Expressao invalida"
            && rpn_resultante != "Parenteses desbalanceados"
        {
            match avaliar_rpn(&rpn_resultante) {
                Ok(valor_resultante) => {
                    println!("Valor esperado: {}", teste.valor_esperado);
                    println!("Valor calculado: {:.6}", valor_resultante);
                    if (valor_resultante - teste.valor_esperado).abs() < 1e-6 {
                        println!("Avaliacao da RPN: OK");
                    } else {
                        println!("Avaliacao da RPN: FALHA");
                    }
                }
                Err(e) => {
                    println!("Erro na avaliação da RPN: {}", e);
                }
            }
        }
        println!("-----------------------");
    }
}

fn main() {
    executar_testes();
}
