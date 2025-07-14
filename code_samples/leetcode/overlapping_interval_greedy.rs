#!/usr/bin/env cargo-script

//! ```cargo
//! [package]
//! name = "overlapping_interval_greedy"
//! version = "0.1.0"
//! edition = "2021"
//!
//! [dependencies]
//! ```

fn atividades(inicios: &[i32], terminos: &[i32]) -> Vec<(i32, i32)> {
    let mut atividades_ordenadas: Vec<(i32, i32)> = inicios.iter().zip(terminos.iter()).map(|(&inicios, &terminos)| (inicios, terminos)).collect();
    atividades_ordenadas.sort_by_key(|&(_, end)| end);
    
    let mut selecionadas = Vec::new();
    if let Some(&first) = atividades_ordenadas.first() {
        selecionadas.push(first);
    }
    
    for &current in atividades_ordenadas.iter().skip(1) {
        if current.0 >= selecionadas.last().unwrap().1 {
            selecionadas.push(current);
        }
    }
    
    selecionadas
}
    

fn main() {
    let inicios = vec![9, 10, 12, 12, 13, 16, 15];
    let terminos = vec![11, 13, 13, 17, 15, 20, 17];
    let resultado = atividades(&inicios, &terminos);
    println!("{:?}", resultado); // [(9,11),(12,13),(13,15),(15,17)]
}
