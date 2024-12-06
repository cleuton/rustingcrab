use rand::Rng;
use std::fmt;
use crate::celula::Celula;
use crate::celula::*;
use crate::pilha::Pilha;

/// Define um labirinto
pub struct Labirinto {
    pub linhas: usize,
    pub colunas: usize,
    pub celulas: Vec<Vec<Celula>>,
    pub valido: bool,
    pub caminho: Option<Vec<(usize, usize)>>,
    _corrente: Option<(usize, usize)>,
    _proxima: Option<(usize, usize)>,
    _qtd_total: usize,
    _qtd_visitadas: usize,
    pilha: Pilha<(usize, usize)>,
}

impl Labirinto {
    /// Cria um novo labirinto
    pub fn new(linhas: usize, colunas: usize) -> Self {
        let mut labirinto = Labirinto {
            linhas,
            colunas,
            celulas: Vec::new(),
            valido: false,
            _corrente: None,
            _proxima: None,
            _qtd_total: 0,
            _qtd_visitadas: 0,
            pilha: Pilha::new(),
            caminho: None,
        };
        labirinto.inicializar();
        labirinto
    }

    /// Inicializa o labirinto
    fn inicializar(&mut self) {
        let mut contador = 0;
        while contador < 4 {
            self.celulas = Vec::new();
            for i in 0..self.linhas {
                let mut linha = Vec::new();
                for j in 0..self.colunas {
                    let celula = Celula {
                        paredes: [true, true, true, true],
                        visitada: false,
                        inicio: false,
                        fim: false,
                        x: j,
                        y: i,
                    };
                    linha.push(celula);
                }
                self.celulas.push(linha);
            }
            self.celulas[0][0].inicio = true;
            self.celulas[self.linhas - 1][self.colunas - 1].fim = true;
            contador += 1;
            self.pilha = Pilha::new();
            self._qtd_visitadas = 0;
            self.criar();
            if !self.fechada(&self.celulas[1][1]) && !self.fechada(&self.celulas[self.linhas - 2][self.colunas - 2]) {
                break;
            }
        }
        if contador < 4 {
            self.valido = true;
        }

    } // fn inicializar

    /// Verifica se a célula está fechada
    fn fechada(&self, celula: &Celula) -> bool {
        celula.paredes[NORTE] && celula.paredes[SUL] && celula.paredes[LESTE] && celula.paredes[OESTE]
    }

    /// Cria o labirinto
    fn criar(&mut self) {
        self._qtd_total = self.linhas * self.colunas;
        let mut rng = rand::thread_rng();
        let linha = rng.gen_range(0..self.linhas);
        let coluna = rng.gen_range(0..self.colunas);
        self._corrente = Some((linha, coluna));
        self.celulas[linha][coluna].visitada = true;
        if let Some(proxima) = self.pegar_vizinha(linha, coluna) {
            self.celulas[proxima.0][proxima.1].visitada = true;
            self.quebrar_paredes((linha, coluna), proxima);
            self.pilha.push((linha, coluna));
            self._qtd_visitadas += 1;
            self._corrente = Some(proxima);
            self.processa_celula();
        }
    }

    /// Processa a célula
    fn processa_celula(&mut self) {
        loop {
            if !self.pilha.is_empty() {
                let corrente = self._corrente.unwrap();
                if self.is_dead_end(corrente) || self.celulas[corrente.0][corrente.1].fim || self.celulas[corrente.0][corrente.1].inicio {
                    if let Some(proxima) = self.pilha.pop() {
                        self._corrente = Some(proxima);
                    } else {
                        self._corrente = None;
                    }
                } else {
                    if let Some(proxima) = self.pegar_vizinha(corrente.0, corrente.1) {
                        self.quebrar_paredes(corrente, proxima);
                        self.pilha.push(corrente);
                        self.celulas[proxima.0][proxima.1].visitada = true;
                        self._qtd_visitadas += 1;
                        self._corrente = Some(proxima);
                    }
                }
            } else {
                self.celulas[0][0].paredes[NORTE] = false;
                self.celulas[self.linhas - 1][self.colunas - 1].paredes[SUL] = false;
                return;
            }
        }
    } // fn processa_celula

    /// Verifica se a célula é um beco sem saída
    fn is_dead_end(&self, celula: (usize, usize)) -> bool {
        let (y, x) = celula;
        if y > 0 && !self.celulas[y - 1][x].visitada {
            return false;
        }
        if y + 1 < self.linhas && !self.celulas[y + 1][x].visitada {
            return false;
        }
        if x > 0 && !self.celulas[y][x - 1].visitada {
            return false;
        }
        if x + 1 < self.colunas && !self.celulas[y][x + 1].visitada {
            return false;
        }
        true
    }

    /// Quebra as paredes entre duas células    
    fn quebrar_paredes(&mut self, c1: (usize, usize), c2: (usize, usize)) {
        let (y1, x1) = c1;
        let (y2, x2) = c2;
        if x1 > x2 {
            self.celulas[y1][x1].paredes[OESTE] = false;
            self.celulas[y2][x2].paredes[LESTE] = false;
        } else if x1 < x2 {
            self.celulas[y1][x1].paredes[LESTE] = false;
            self.celulas[y2][x2].paredes[OESTE] = false;
        } else if y1 > y2 {
            self.celulas[y1][x1].paredes[NORTE] = false;
            self.celulas[y2][x2].paredes[SUL] = false;
        } else if y1 < y2 {
            self.celulas[y1][x1].paredes[SUL] = false;
            self.celulas[y2][x2].paredes[NORTE] = false;
        }
    }

    /// Pega uma célula vizinha
    fn pegar_vizinha(&self, y: usize, x: usize) -> Option<(usize, usize)> {
        let mut procurar = true;
        let mut cel = None;
        let mut rng = rand::thread_rng();
        while procurar {
            let vizinha = rng.gen_range(0..4);
            match vizinha {
                NORTE => {
                    if y > 0 && !self.celulas[y - 1][x].visitada {
                        cel = Some((y - 1, x));
                        procurar = false;
                    }
                }
                SUL => {
                    if y + 1 < self.linhas && !self.celulas[y + 1][x].visitada {
                        cel = Some((y + 1, x));
                        procurar = false;
                    }
                }
                LESTE => {
                    if x + 1 < self.colunas && !self.celulas[y][x + 1].visitada {
                        cel = Some((y, x + 1));
                        procurar = false;
                    }
                }
                OESTE => {
                    if x > 0 && !self.celulas[y][x - 1].visitada {
                        cel = Some((y, x - 1));
                        procurar = false;
                    }
                }
                _ => {}
            }
        }
        cel
    } // fn pegar_vizinha
            
    /// Método auxiliar para obter a representação de uma célula            
    fn get_celula(&self, cel: &Celula) -> Vec<Vec<char>> {
        let mut linha1 = vec![' ', ' ', ' '];
        let mut linha2 = vec![' ', ' ', ' '];
        let mut linha3 = vec![' ', ' ', ' '];
        if cel.paredes[NORTE] {
            linha1 = vec!['-', '-', '-'];
        }
        if cel.paredes[SUL] {
            linha3 = vec!['-', '-', '-'];
        }
        if cel.paredes[OESTE] {
            linha1[0] = if linha1[0] == '-' { '+' } else { '|' };
            linha2[0] = '|';
            linha3[0] = if linha3[0] == '-' { '+' } else { '|' };
        }
        if cel.paredes[LESTE] {
            linha1[2] = if linha1[2] == '-' { '+' } else { '|' };
            linha2[2] = '|';
            linha3[2] = if linha3[2] == '-' { '+' } else { '|' };
        }
        // Marca o caminho se existir
        if let Some(ref caminho) = self.caminho {
            if caminho.contains(&(cel.y, cel.x)) {
                linha2[1] = '*';
            }
        }
        vec![linha1, linha2, linha3]
    } // fn get_celula

    /// Insere uma célula na matriz
    fn insert(&self, linhas: &mut Vec<Vec<char>>, matriz: &Vec<Vec<char>>, i: usize, j: usize) {
        let linha = i * 2;
        let coluna = j * 2;
        for l in 0..3 {
            for c in 0..3 {
                linhas[linha + l][coluna + c] = matriz[l][c];
            }
        }
    }
}

/// Implementação do Display para imprimir o labirinto
impl fmt::Display for Labirinto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut linhas = vec![vec![' '; (self.colunas * 3) + 1]; self.linhas * 3];
        for z in linhas.iter_mut() {
            z[self.colunas * 3] = '\n';
        }
        for i in 0..self.linhas {
            for j in 0..self.colunas {
                let matriz = self.get_celula(&self.celulas[i][j]);
                self.insert(&mut linhas, &matriz, i, j);
            }
        }
        for linha in linhas {
            for c in linha {
                write!(f, "{}", c)?;
            }
        }
        Ok(())
    }
}

