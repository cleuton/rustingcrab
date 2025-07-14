#!/usr/bin/env cargo-script

//! ```cargo
//! [package]
//! name = "monotonic_stack"
//! version = "0.1.0"
//! edition = "2021"
//!
//! [dependencies]
//! ```

fn encontrar_proximo_maior(numeros: &[i32]) -> Vec<i32> {
    let mut resultado = vec![-1; numeros.len()];
    let mut pilha = Vec::new();

    for i in (0..numeros.len()).rev() {
        let numero = numeros[i];

        // Remove elementos menores ou iguais ao atual
        while let Some(&topo) = pilha.last() {
            if topo <= numero {
                pilha.pop();
            } else {
                break;
            }
        }

        // Se ainda há elementos, o topo é o próximo maior
        if let Some(&topo) = pilha.last() {
            resultado[i] = topo;
        }

        // Adiciona o número atual à pilha
        pilha.push(numero);
    }

    resultado
}

fn main() {
    let numeros = vec![2, 1, 2, 4, 3];
    let resultado = encontrar_proximo_maior(&numeros);
    println!("{:?}", resultado); // Saída: [4, 2, 4, -1, -1]
}