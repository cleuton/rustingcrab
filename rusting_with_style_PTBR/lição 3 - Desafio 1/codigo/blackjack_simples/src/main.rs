
use std::io;
use rand::Rng;

fn main() {
    let num_decks = 4;
    let mut deck = criar_baralho(num_decks);

    // Agora a leitura será extremamente simples, sem expect, sem match, sem nada sofisticado:
    // Apenas lê a linha, parse com unwrap e confia que o usuário digitou corretamente.
    println!("Digite o valor inicial para apostar:");
    let dinheiro_jogador_inicial = le_inteiro();
    let mut dinheiro_jogador = dinheiro_jogador_inicial;

    loop {
        if deck.len() < 10 {
            println!("Acabaram as cartas, jogo encerrado!");
            break;
        }
        if dinheiro_jogador < 1 {
            println!("Você ficou sem dinheiro, jogo encerrado!");
            break;
        }

        println!("Você tem {} de dinheiro. Deseja continuar jogando? (s/n)", dinheiro_jogador);
        let continuar = le_string();
        if continuar.trim().to_lowercase() != "s" {
            println!("Jogo encerrado por opção do jogador!");
            break;
        }

        println!("Digite o valor da aposta (min 1):");
        let aposta = le_inteiro();
        if aposta < 1 || aposta > dinheiro_jogador {
            println!("Aposta inválida. Tente novamente.");
            continue;
        }

        let mut mao_jogador = Vec::new();
        let mut mao_mesa = Vec::new();

        comprar_carta(&mut deck, &mut mao_jogador);
        comprar_carta(&mut deck, &mut mao_jogador);
        comprar_carta(&mut deck, &mut mao_mesa);
        comprar_carta(&mut deck, &mut mao_mesa);

        // Turno do jogador
        loop {
            let p_jogador = pontuacao_jogador(mao_jogador.clone());
            println!("Suas cartas: {:?} (pontuação: {})", mao_jogador, p_jogador);
            if p_jogador > 21 {
                println!("Você estourou! Pontos: {}", p_jogador);
                dinheiro_jogador -= aposta;
                break;
            }

            println!("Deseja comprar mais cartas? (s/n)");
            let opcao = le_string();
            if opcao.trim().to_lowercase() != "s" {
                break;
            }

            comprar_carta(&mut deck, &mut mao_jogador);
        }

        let p_jogador = pontuacao_jogador(mao_jogador.clone());
        if p_jogador > 21 {
            continue;
        }

        // Turno da mesa
        loop {
            let p_mesa = pontuacao_mesa(mao_mesa.clone());
            if p_mesa > 21 {
                println!("Mesa estourou! Pontos da mesa: {}", p_mesa);
                dinheiro_jogador += aposta;
                break;
            }

            if p_mesa >= 17 {
                println!("A mesa parou de comprar. Pontos da mesa: {}", p_mesa);
                if p_mesa > p_jogador {
                    println!("A mesa ganhou! {} vs {}", p_mesa, p_jogador);
                    dinheiro_jogador -= aposta;
                } else if p_mesa < p_jogador {
                    println!("Você ganhou! {} vs {}", p_jogador, p_mesa);
                    dinheiro_jogador += aposta;
                } else {
                    println!("Empate! Ninguém ganha ou perde.");
                }
                break;
            } else {
                comprar_carta(&mut deck, &mut mao_mesa);
            }
        }
    }

    println!("Você terminou com {} de dinheiro.", dinheiro_jogador);
    println!("Obrigado por jogar!");
}

fn criar_baralho(num_decks: usize) -> Vec<i32> {
    let mut deck = Vec::new();
    for _ in 0..num_decks {
        for _ in 0..4 {
            for carta in 1..=13 {
                deck.push(carta);
            }
        }
    }
    deck
}

fn comprar_carta(deck: &mut Vec<i32>, mao: &mut Vec<i32>) {
    if deck.is_empty() {
        return;
    }
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..deck.len());
    let carta = deck.remove(idx);
    mao.push(carta);
}

fn pontuacao_jogador(mao: Vec<i32>) -> i32 {
    let mut total = 0;
    for carta in mao {
        total += valor_carta(carta);
    }
    total
}

fn pontuacao_mesa(mao: Vec<i32>) -> i32 {
    let mut total = 0;
    for carta in mao {
        total += valor_carta(carta);
    }
    total
}

fn valor_carta(carta: i32) -> i32 {
    if carta >= 11 && carta <= 13 {
        10 // equivale a "return 10;"
    } else {
        carta // equivale a "return carta;"
    }
}

// Leitura extremamente simples de um inteiro do stdin, sem expect, sem match, sem nada sofisticado.
// Confia-se no usuário que irá digitar um número válido.
fn le_inteiro() -> i32 {
    let mut entrada = String::new();
    let _ = io::stdin().read_line(&mut entrada); // Ignora o Result
    let numero: i32 = entrada.trim().parse().unwrap();
    numero // equivale a "return numero;"
}

fn le_string() -> String {
    let mut entrada = String::new();
    let _ = io::stdin().read_line(&mut entrada); // Ignora o Result
    entrada // equivale a "return entrada;"
}

