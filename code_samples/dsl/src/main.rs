use std::env;
use logos::Logos;
use nom::{
    branch::alt,
    multi::fold_many0,
    sequence::{delimited, pair},
    IResult,
};

// Definição dos tokens
#[derive(Logos, Debug, PartialEq, Clone)]
enum Token {
    #[regex(r"[0-9]+", |lex| lex.slice().parse())]
    Numero(i32),

    #[token("+")]
    Mais,
    #[token("-")]
    Menos,
    #[token("*")]
    Multiplicacao,
    #[token("/")]
    Divisao,

    #[token("(")]
    AbreParenteses,
    #[token(")")]
    FechaParenteses,

    #[regex(r"[ \t\n\f]+", logos::skip)] // Ignorar espaços
    Whitespace,

    #[error]
    Error,
}

// Definição da AST
#[derive(Debug, Clone)]
enum Expr {
    Numero(i32),
    Operacao {
        op: Token,
        esquerda: Box<Expr>,
        direita: Box<Expr>,
    },
}

// Parser para números
fn parse_numero(tokens: &[Token]) -> IResult<&[Token], Expr> {
    if let Some((Token::Numero(valor), rest)) = tokens.split_first() {
        Ok((rest, Expr::Numero(*valor)))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(tokens, nom::error::ErrorKind::Tag)))
    }
}

// Parser para fatores (números ou expressões entre parênteses)
fn parse_fator(tokens: &[Token]) -> IResult<&[Token], Expr> {
    alt((
        parse_numero,
        delimited(
            parse_abre_parenteses,
            parse_expr,
            parse_fecha_parenteses,
        ),
    ))(tokens)
}

fn parse_abre_parenteses(tokens: &[Token]) -> IResult<&[Token], ()> {
    if let Some((Token::AbreParenteses, rest)) = tokens.split_first() {
        Ok((rest, ()))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(tokens, nom::error::ErrorKind::Tag)))
    }
}

fn parse_fecha_parenteses(tokens: &[Token]) -> IResult<&[Token], ()> {
    if let Some((Token::FechaParenteses, rest)) = tokens.split_first() {
        Ok((rest, ()))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(tokens, nom::error::ErrorKind::Tag)))
    }
}

// Parser para termos (multiplicação e divisão)
fn parse_termo(tokens: &[Token]) -> IResult<&[Token], Expr> {
    let (tokens, inicial) = parse_fator(tokens)?;
    fold_many0(
        pair(parse_multiplicacao_ou_divisao, parse_fator),
        move || inicial.clone(),
        |esquerda, (op, direita)| Expr::Operacao {
            op,
            esquerda: Box::new(esquerda),
            direita: Box::new(direita),
        },
    )(tokens)
}

fn parse_multiplicacao_ou_divisao(tokens: &[Token]) -> IResult<&[Token], Token> {
    if let Some((token @ Token::Multiplicacao, rest)) = tokens.split_first() {
        Ok((rest, token.clone()))
    } else if let Some((token @ Token::Divisao, rest)) = tokens.split_first() {
        Ok((rest, token.clone()))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(tokens, nom::error::ErrorKind::Tag)))
    }
}

// Parser para expressões completas (adição e subtração)
fn parse_expr(tokens: &[Token]) -> IResult<&[Token], Expr> {
    let (tokens, inicial) = parse_termo(tokens)?;
    fold_many0(
        pair(parse_mais_ou_menos, parse_termo),
        move || inicial.clone(),
        |esquerda, (op, direita)| Expr::Operacao {
            op,
            esquerda: Box::new(esquerda),
            direita: Box::new(direita),
        },
    )(tokens)
}

fn parse_mais_ou_menos(tokens: &[Token]) -> IResult<&[Token], Token> {
    if let Some((token @ Token::Mais, rest)) = tokens.split_first() {
        Ok((rest, token.clone()))
    } else if let Some((token @ Token::Menos, rest)) = tokens.split_first() {
        Ok((rest, token.clone()))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(tokens, nom::error::ErrorKind::Tag)))
    }
}

// Avaliador da AST
fn avaliar(expr: &Expr) -> i32 {
    match expr {
        Expr::Numero(valor) => *valor,
        Expr::Operacao { op, esquerda, direita } => {
            let esq = avaliar(esquerda);
            let dir = avaliar(direita);
            match op {
                Token::Mais => esq + dir,
                Token::Menos => esq - dir,
                Token::Multiplicacao => esq * dir,
                Token::Divisao => esq / dir,
                _ => panic!("Operador inválido"),
            }
        }
    }
}

// cargo run "3 + 5 * (2 - 8)"    => -27
// cargo run "(3 + 12) * (4 - 3)" => 15

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Uso: {} <expressão>", args[0]);
        std::process::exit(1);
    }

    let expressao = &args[1];
    let tokens: Vec<_> = Token::lexer(expressao).collect();

    match parse_expr(&tokens) {
        Ok((_, ast)) => {
            println!("AST: {:?}", ast);
            println!("Resultado: {}", avaliar(&ast));
        }
        Err(err) => println!("Erro ao parsear: {:?}", err),
    }
}
