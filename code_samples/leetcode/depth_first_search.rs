#!/usr/bin/env cargo-script

//! ```cargo
//! [package]
//! name = "depth_first_search"
//! version = "0.1.0"
//! edition = "2021"
//!
//! [dependencies]
//! ```

use std::collections::{HashMap, HashSet};

/// Verifica via DFS se existe caminho direcionado de `origem` a `destino`
fn existe_caminho_dfs(arestas: &[(char, char)], origem: char, destino: char) -> bool {
    // Monta lista de adjacência
    let mut grafo: HashMap<char, Vec<char>> = HashMap::new();
    for &(de, para) in arestas {
        grafo.entry(de).or_default().push(para);
    }
    // Conjunto de visitados para evitar loops
    let mut visitadas = HashSet::new();
    // Chama DFS inicial
    dfs(origem, destino, &grafo, &mut visitadas)
}

fn dfs(
    atual: char,
    destino: char,
    grafo: &HashMap<char, Vec<char>>,
    visitadas: &mut HashSet<char>,
) -> bool {
    if atual == destino {
        return true;
    }
    // Marca antes de recursão para não revisitar
    visitadas.insert(atual);
    // Explora cada vizinho
    if let Some(vizinhos) = grafo.get(&atual) {
        for &vizinho in vizinhos {
            if !visitadas.contains(&vizinho) {
                if dfs(vizinho, destino, grafo, visitadas) {
                    return true;
                }
            }
        }
    }
    false
}

fn main() {
    let rede = [
        ('b','f'), ('a','b'), ('d','e'),
        ('a','c'), ('c','d'), ('f','e'),
        ('b','c'), ('j','k'), ('c','e'),
    ];
    println!("{}", existe_caminho_dfs(&rede, 'a', 'e')); // true
    println!("{}", existe_caminho_dfs(&rede, 'a', 'k')); // false

    let rede2 = [
        ('d','f'), ('d','c'), ('c','e'),
        ('b','d'), ('b','c'), ('a','b'), ('f','e'),
    ];
    println!("{}", existe_caminho_dfs(&rede2, 'a', 'e')); // true
}
