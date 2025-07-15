#!/usr/bin/env cargo-script

//! ```cargo
//! [package]
//! name = "a_star"
//! version = "0.1.0"
//! edition = "2021"
//!
//! [dependencies]
//! ```

use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct No {
    f_score: usize,
    pos: Pos,
}

// Impl Ord define como nós No são comparados dentro do BinaryHeap.
// Aqui invertemos a comparação de f_score para simular um min-heap
// (já que o BinaryHeap padrão é max-heap). Em caso de empate em f_score,
// desempata-se pelas coordenadas x e depois y para estabilidade.
impl Ord for No {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compara f_score invertido
        let cmp_fs = other.f_score.cmp(&self.f_score);
        if cmp_fs != Ordering::Equal {
            return cmp_fs;
        }
        // Desempate por x
        let cmp_x = self.pos.x.cmp(&other.pos.x);
        if cmp_x != Ordering::Equal {
            return cmp_x;
        }
        // Desempate por y
        self.pos.y.cmp(&other.pos.y)
    }
}

// PartialOrd simplesmente delega a Ord acima,
// permitindo comparações parciais necessárias ao heap.
impl PartialOrd for No {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Calcula a distância de Manhattan entre os pontos `a` e `b`.
///
/// A distância de Manhattan é a soma das diferenças absolutas
/// das coordenadas x e y:
///
///     |a.x − b.x| + |a.y − b.y|
///
/// Em um grid 4‑conectado (movimentos ortogonais),
//  esse valor corresponde ao menor número de passos
/// necessários para ir de `a` a `b`.
///
/// Propriedades em A*:
/// - Admissível: nunca superestima o custo real do caminho.
/// - Consistente: satisfaz a desigualdade triangular,
///   garantindo que o f_score só aumente ao longo do caminho.
fn heuristica(a: Pos, b: Pos) -> usize {
    let dx = if a.x > b.x { a.x - b.x } else { b.x - a.x };
    let dy = if a.y > b.y { a.y - b.y } else { b.y - a.y };
    dx + dy
}

fn vizinhos(pos: Pos, largura: usize, altura: usize) -> Vec<Pos> {
    let mut v = Vec::new();
    if pos.x > 0              { v.push(Pos { x: pos.x - 1, y: pos.y }); }
    if pos.x + 1 < largura    { v.push(Pos { x: pos.x + 1, y: pos.y }); }
    if pos.y > 0              { v.push(Pos { x: pos.x,     y: pos.y - 1 }); }
    if pos.y + 1 < altura     { v.push(Pos { x: pos.x,     y: pos.y + 1 }); }
    v
}

fn a_star(grid: &Vec<Vec<u8>>, inicio: Pos, alvo: Pos) -> Option<Vec<Pos>> {
    let largura = grid[0].len();
    let altura = grid.len();

    // open_set: heap de nós a explorar, ordenado por f_score
    let mut open_set = BinaryHeap::new();
    open_set.push(No { f_score: heuristica(inicio, alvo), pos: inicio });

    // came_from registra o predecessor de cada nó
    let mut came_from = HashMap::new();
    // g_score: custo do início até cada nó
    let mut g_score = HashMap::new();
    g_score.insert(inicio, 0);

    while let Some(No { pos: atual, .. }) = open_set.pop() {
        if atual == alvo {
            // Reconstrói caminho de trás para frente
            let mut caminho = Vec::new();
            let mut cur = atual;
            while cur != inicio {
                caminho.push(cur);
                cur = came_from[&cur];
            }
            caminho.push(inicio);
            caminho.reverse();
            return Some(caminho);
        }

        for viz in vizinhos(atual, largura, altura) {
            if grid[viz.y][viz.x] != 0 { continue; } // obstáculo

            let tent_g = g_score.get(&atual).unwrap_or(&usize::MAX) + 1;
            if tent_g < *g_score.get(&viz).unwrap_or(&usize::MAX) {
                came_from.insert(viz, atual);
                g_score.insert(viz, tent_g);
                let f = tent_g + heuristica(viz, alvo);
                open_set.push(No { f_score: f, pos: viz });
            }
        }
    }

    None // sem caminho
}

fn main() {
    // 0 = livre, 1 = obstáculo
    let grid = vec![
        vec![0, 0, 0, 0, 1],
        vec![1, 1, 0, 0, 0],
        vec![0, 0, 0, 1, 1],
        vec![0, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
    ];
    let inicio = Pos { x: 0, y: 0 };
    let alvo   = Pos { x: 4, y: 4 };

    if let Some(caminho) = a_star(&grid, inicio, alvo) {
        // converte grid em chars: '.' livre, '#' obstáculo
        let mut mapa: Vec<Vec<char>> = grid.iter()
            .map(|linha| linha.iter()
                .map(|&v| if v == 0 { '.' } else { '#' })
                .collect()
            )
            .collect();

        // marca o caminho com '*'
        for &p in &caminho {
            mapa[p.y][p.x] = '*';
        }
        // marca início como 'I' e alvo como 'F'
        mapa[inicio.y][inicio.x] = 'I';
        mapa[alvo.y][alvo.x]     = 'F';

        // imprime o mapa final
        for linha in mapa {
            let s: String = linha.into_iter().collect();
            println!("{}", s);
        }
    } else {
        println!("Alvo inalcançável");
    }
}
