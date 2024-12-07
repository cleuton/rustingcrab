use crate::carta::Carta;
use rand::Rng;

pub struct Jogo {
    pub baralho: Vec<Carta>,
    pub mao_do_jogador: Vec<Carta>,
    pub pontos_do_jogador: i32,
    pub mao_do_croupier: Vec<Carta>,
    pub aposta_jogador: i32,
    pub jogador_ganhou: bool,
    pub jogador_perdeu: bool,
}

fn get_baralho(qtde_baralhos: u8) -> Vec<Carta> {
    // Adicionando a quantidade de baralhos especificada
    let mut cartas: Vec<Carta> = Vec::new();
    let naipes = vec!["♠", "♥", "♦", "♣"];
    for _ in 0..qtde_baralhos {
        for naipe in &naipes {
            // código para adicionar cartas do naipe ao baralho
            for carta in 1..=10 {
                let mut valor = carta;
                if carta > 10 {
                    valor = 10;
                } 
                cartas.push(Carta {valor: valor, naipe: naipe.to_string()});
            }
        }
    }
    cartas
}

impl Jogo { 
    pub fn new(qtde_baralhos: u8, pontos_para_jogador: i32) -> Self {
        Jogo {
            baralho: get_baralho(qtde_baralhos),
            mao_do_jogador: Vec::new(),
            pontos_do_jogador: pontos_para_jogador,
            mao_do_croupier: Vec::new(),
            aposta_jogador: 0,
            jogador_ganhou: false,
            jogador_perdeu: false,
        }
    }

    pub fn reiniciar(&mut self, qtde_baralhos: u8, pontos_para_jogador: i32) {
        self.baralho = get_baralho(qtde_baralhos);
        self.mao_do_croupier = Vec::new();
        self.mao_do_jogador = Vec::new();
        self.pontos_do_jogador = pontos_para_jogador;
        self.aposta_jogador = 0;
        self.jogador_ganhou = false;
        self.jogador_perdeu = false;
    }

    pub fn jogador_aposta(&mut self, valor: i32) -> bool {
        if self.pontos_do_jogador >= valor {
            self.aposta_jogador = valor;
            self.pontos_do_jogador -= valor;
            return true;
        }
        false
    }

    pub fn jogador_compra(&mut self) -> bool {
        if self.baralho.len() > 0 {
            let indice = rand::thread_rng().gen_range(0..self.baralho.len());
            self.mao_do_jogador.push(self.baralho[indice].clone());
            self.baralho.remove(indice);
            return true;
        }
        false
    }

    pub fn mesa_compra(&mut self) -> bool {
        if self.baralho.len() > 0 {
            let indice = rand::thread_rng().gen_range(0..self.baralho.len());
            self.mao_do_croupier.push(self.baralho[indice].clone());
            self.baralho.remove(indice);
            return true;
        }
        false   
    }

    pub fn calcular_mao(&self, mao: &Vec<Carta>) -> i32 {
        let mut valor = 0;
        for carta in mao {
            valor += carta.valor;
        }
        valor
    } 

    pub fn nova_rodada(&mut self) {
        self.mao_do_jogador = Vec::new();
        self.mao_do_croupier = Vec::new();
        self.aposta_jogador = 0;
        self.jogador_ganhou = false;
        self.jogador_perdeu = false;
    }

    pub fn imprime_mao(&self, mao: &Vec<Carta>) -> String {
        let mut cartas = String::new();
        for carta in mao {
            cartas.push_str(&carta.desenhar());
        }
        cartas
    }
}