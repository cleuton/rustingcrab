use crate::pilha::Pilha;
use crate::labirinto::Labirinto;

/// Implementação de um solucionador de labirintos.
pub struct Solver {
    pilha: Pilha<(usize, usize)>,
    corrente: Option<(usize, usize)>,
    caminho: Vec<(usize, usize)>,
    incrementos: Vec<(isize, isize)>,
    visitadas: Vec<Vec<bool>>,
}

impl Solver {
    pub fn new() -> Self {
        Solver {
            pilha: Pilha::new(),
            corrente: None,
            caminho: Vec::new(),
            incrementos: vec![(0, -1), (0, 1), (1, 0), (-1, 0)],
            visitadas: Vec::new(),
        }
    }

    pub fn solve(&mut self, labirinto: &mut Labirinto) {
        self.visitadas = vec![vec![false; labirinto.colunas]; labirinto.linhas];
        self.corrente = Some((0, 0));
        self.pilha.push((0, 0));
        self.procurar(labirinto);
        while let Some(pos) = self.pilha.pop() {
            self.caminho.push(pos);
        }
        labirinto.caminho = Some(self.caminho.clone());
    }

    fn procurar(&mut self, labirinto: &Labirinto) {
        let mut buffer = None;
        while !self.pilha.is_empty() {
            let (y, x) = *self.pilha.top().unwrap();
            self.visitadas[y][x] = true;
            if labirinto.celulas[y][x].fim {
                return;
            }
            let mut proxima = None;
            for parede in 0..4 {
                if !labirinto.celulas[y][x].paredes[parede] {
                    let dy = self.incrementos[parede].1;
                    let dx = self.incrementos[parede].0;
                    let ny = y as isize + dy;
                    let nx = x as isize + dx;
                    if ny < 0 || nx < 0 || ny as usize >= labirinto.linhas || nx as usize >= labirinto.colunas {
                        continue;
                    }
                    let ny = ny as usize;
                    let nx = nx as usize;
                    if self.visitadas[ny][nx] {
                        continue;
                    }
                    proxima = Some((ny, nx));
                    if let Some(pos) = buffer {
                        self.pilha.push(pos);
                        buffer = None;
                    }
                    self.pilha.push((ny, nx));
                    self.corrente = Some((ny, nx));
                    break;
                }
            }
            if proxima.is_none() {
                let pos = self.pilha.pop().unwrap();
                self.corrente = Some(pos);
                buffer = Some(pos);
            }
        }
    }
}
