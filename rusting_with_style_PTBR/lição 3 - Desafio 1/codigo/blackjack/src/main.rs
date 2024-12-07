use blackjack::Jogo;
use std::io;
use std::io::prelude::*;

fn pergunta(msg: &str) -> bool {
    print!("{} (s/n)?", msg);
    let mut resp = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut resp).unwrap();
    if resp.trim().to_uppercase() == "S" {
        return true;
    }
    false
}

fn pede_aposta(maximo: i32) -> i32 {
    let mut aposta: i32;
    loop {        
        let mut valor = String::new();
        print!("Quanto quer apostar? Máximo: {}, Zero termina o jogo.", maximo);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut valor).unwrap();
        aposta = valor.trim().parse().unwrap();
        if aposta == 0 {
            return 0;
        }
        if aposta > maximo {
            println!("O valor máximo para aposta é {}", maximo);
            continue;
        }
        break;
    }
    aposta
}
 
fn jogada_do_jogador(jogo: &mut Jogo) -> bool {
    if jogo.pontos_do_jogador <= 0 {
        return false;
    }
    println!("Sua mão: {} valor: {}", jogo.imprime_mao(&jogo.mao_do_jogador), jogo.calcular_mao(&jogo.mao_do_jogador));
    println!("Seus pontos: {}", jogo.pontos_do_jogador);
    if jogo.aposta_jogador == 0 {
        jogo.aposta_jogador = pede_aposta(jogo.pontos_do_jogador);
        if jogo.aposta_jogador == 0 {
            // Não quer mais jogar
            return false;
        }
    }
    loop {
        if jogo.jogador_compra() {
            println!("Sua mão: {} valor: {}", jogo.imprime_mao(&jogo.mao_do_jogador), jogo.calcular_mao(&jogo.mao_do_jogador));
            let valor_mao = jogo.calcular_mao(&jogo.mao_do_jogador);
            if  valor_mao == 21 {
                jogo.pontos_do_jogador += jogo.aposta_jogador;
                println!("Você ganhou. ");
                jogo.jogador_ganhou = true;
                return true;
            }
            if valor_mao > 21 {
                jogo.pontos_do_jogador -= jogo.aposta_jogador;
                println!("Você Perdeu. ");
                jogo.jogador_perdeu = true;
                return true;               
            }
            if !pergunta("Quer comprar mais ") {
                break;
            }
        } else {
            return false;
        }
    }
    return true;
}

fn verifica_pontos(jogo: &mut Jogo) {
    let valor_mao_jogador = jogo.calcular_mao(&jogo.mao_do_jogador);
    let valor_mao_mesa = jogo.calcular_mao(&jogo.mao_do_croupier);
    if valor_mao_jogador > valor_mao_mesa {
        println!("Você ganhou. ");
        jogo.pontos_do_jogador += jogo.aposta_jogador;
    } else if valor_mao_jogador < valor_mao_mesa {
        println!("Você perdeu. ");
        jogo.pontos_do_jogador -= jogo.aposta_jogador;
    } else {
        println!("Empate. ");
    }
}

fn jogada_da_mesa(jogo: &mut Jogo) -> bool {
    println!("Mão da mesa: {} valor: {}", jogo.imprime_mao(&jogo.mao_do_croupier), jogo.calcular_mao(&jogo.mao_do_croupier));
    loop {
        if jogo.mesa_compra() {
            println!("Mão da mesa: {} valor: {}", jogo.imprime_mao(&jogo.mao_do_croupier), jogo.calcular_mao(&jogo.mao_do_croupier));
            let valor_mao = jogo.calcular_mao(&jogo.mao_do_croupier);
            if  valor_mao == 21 {
                jogo.pontos_do_jogador -= jogo.aposta_jogador;
                jogo.jogador_perdeu = true;
                println!("Você perdeu a mesa fez 21. s");
                return true;
            }
            if valor_mao > 21 {
                jogo.pontos_do_jogador += jogo.aposta_jogador;
                jogo.jogador_ganhou = true;
                println!("Você Ganhou a mesa passou de 21. ");
                return true;               
            }
            if valor_mao > 18 {
                break;
            }
        } else {
            return false;
        }
    }
    return true;
}

fn main() {
    println!("Vamos jogar black jack!");
    let qtde_baralhos = 4;
    let pontos_jogador = 10;
    let mut jogo = Jogo::new(qtde_baralhos, pontos_jogador);
    loop {
        // Nova rodada começando pelo jogador
        println!("Nova rodada. Vez do jogador. Pontos: {}", jogo.pontos_do_jogador);
        jogo.nova_rodada();
        if !jogada_do_jogador(&mut jogo) {
            // Ou acabou o baralho ou o jogador perdeu tudo
            break;
        }
        // Se o jogador já ganhou ou perdeu, então a mesa não joga
        if !jogo.jogador_ganhou && !jogo.jogador_perdeu {
            // Agora é a mesa
            if !jogada_da_mesa(&mut jogo) {
                // Acabou o baralho
                break;
            }
            if !jogo.jogador_ganhou && !jogo.jogador_perdeu {
                verifica_pontos(&mut jogo);
            }
        }

        if jogo.pontos_do_jogador <= 0 {
            println!("Você perdeu todos os pontos. ");
            break;
        }

        if !pergunta("Mais uma rodada ") {
            break;
        }
    }
    // Acabou o jogo
    println!("Seus pontos totais: {}", jogo.pontos_do_jogador);
}
