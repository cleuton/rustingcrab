#!/usr/bin/env cargo-script

//! ```cargo
//! [package]
//! name = "matrix_transversal"
//! version = "0.1.0"
//! edition = "2021"
//!
//! [dependencies]
//! ```

/// Conta quantas “ilhas” existem em uma grade 2D de '1' (terra) e '0' (água),
/// usando DFS para “inundar” cada ilha encontrada.
fn contar_ilhas(grid: &mut Vec<Vec<char>>) -> usize {
    let linhas = grid.len();
    if linhas == 0 { return 0; }
    let colunas = grid[0].len();
    let mut total = 0;

    for i in 0..linhas {
        for j in 0..colunas {
            if grid[i][j] == '1' {
                total += 1;
                inundar_ilha(grid, i, j, linhas, colunas);
            }
        }
    }
    total
}

/// Marca recursivamente todas as células conectadas à ilha em (i, j) como '0'
fn inundar_ilha(
    grid: &mut Vec<Vec<char>>,
    i: usize,
    j: usize,
    linhas: usize,
    colunas: usize,
) {
    // Se está fora dos limites ou já é água, nada a fazer
    if i >= linhas || j >= colunas || grid[i][j] != '1' {
        return;
    }
    // “Afoga” a célula, evitando revisitas
    grid[i][j] = '0';

    // Tenta as quatro direções
    if i > 0 {
        inundar_ilha(grid, i - 1, j, linhas, colunas);
    }
    if i + 1 < linhas {
        inundar_ilha(grid, i + 1, j, linhas, colunas);
    }
    if j > 0 {
        inundar_ilha(grid, i, j - 1, linhas, colunas);
    }
    if j + 1 < colunas {
        inundar_ilha(grid, i, j + 1, linhas, colunas);
    }
}

fn main() {
    let mut exemplo1 = vec![
        vec!['1','1','0','0','0'],
        vec!['1','1','0','0','0'],
        vec!['0','0','1','0','0'],
        vec!['0','0','0','1','1'],
    ];
    println!("Ilhas no exemplo1: {}", contar_ilhas(&mut exemplo1)); // 3

    let mut exemplo2 = vec![
        vec!['1','0','1','1'],
        vec!['1','0','0','1'],
        vec!['0','0','0','0'],
    ];
    println!("Ilhas no exemplo2: {}", contar_ilhas(&mut exemplo2)); // 2
}
