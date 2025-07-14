#!/usr/bin/env cargo-script

//! ```cargo
//! [package]
//! name = "breadth_first_search"
//! version = "0.1.0"
//! edition = "2021"
//!
//! [dependencies]
//! ```

use std::collections::{HashMap, VecDeque};

fn menor_caminho(arestas: &[(char, char)], inicio: char, fim: char) -> Vec<char> {
    // Monta lista de adjacência
    let mut grafo: HashMap<char, Vec<char>> = HashMap::new();
    for &(a, b) in arestas {
        grafo.entry(a).or_default().push(b);
        grafo.entry(b).or_default().push(a);
    }

    // Mapeia cada nó ao seu predecessor na busca
    let mut anteriores: HashMap<char, char> = HashMap::new();
    // Fila para o BFS
    let mut fila: VecDeque<char> = VecDeque::new();
    // Marca nós visitados
    let mut visitados: HashMap<char, bool> = HashMap::new();

    fila.push_back(inicio);
    visitados.insert(inicio, true);

    // Executa BFS até encontrar `fim` ou esgotar
    while let Some(cidade) = fila.pop_front() {
        if cidade == fim {
            // Reconstrói caminho de trás pra frente
            let mut caminho = vec![fim];
            let mut atual = fim;
            while let Some(&ant) = anteriores.get(&atual) {
                caminho.push(ant);
                atual = ant;
            }
            caminho.reverse();
            return caminho;
        }
        for &vizinho in &grafo[&cidade] {
            if !visitados.get(&vizinho).copied().unwrap_or(false) {
                visitados.insert(vizinho, true);
                anteriores.insert(vizinho, cidade);
                fila.push_back(vizinho);
            }
        }
    }

    // Se não encontrou rota
    Vec::new()
}

fn main() {
    let rede1 = [
        ('b','f'), ('a','b'), ('d','e'),
        ('a','c'), ('c','d'), ('f','e'),
        ('b','c'), ('c','e'),
    ];
    let rota1 = menor_caminho(&rede1, 'a', 'e');
    println!("{:?}", rota1); // ['a', 'c', 'e']

    let rede2 = [
        ('d','f'), ('d','c'), ('c','e'),
        ('b','d'), ('b','c'), ('a','b'), ('f','e'),
    ];
    let rota2 = menor_caminho(&rede2, 'a', 'e');
    println!("{:?}", rota2); // ['a', 'b', 'c', 'e']
}
