#!/usr/bin/env cargo-script

//! ```cargo
//! [package]
//! name = "dynamic_programming"
//! version = "0.1.0"
//! edition = "2021"
//!
//! [dependencies]
//! ```

/// Calcula a maior soma de elementos não adjacentes em um vetor.
fn maior_soma_sem_adjacentes(valores: &[i32]) -> i32 {
    let n = valores.len();
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return valores[0];
    }

    // dp[i] = maior soma possível considerando até o índice i
    let mut dp = vec![0; n];
    dp[0] = valores[0];
    // para o segundo elemento, escolhe o maior entre o primeiro e o segundo
    dp[1] = valores[0].max(valores[1]);

    // preenche o dp a partir do terceiro elemento
    for i in 2..n {
        // se pular o elemento i, fica com dp[i-1];
        // se escolher o elemento i, soma dp[i-2] + valores[i]
        dp[i] = dp[i - 1].max(dp[i - 2] + valores[i]);
    }

    // a resposta é o último valor do dp
    dp[n - 1]
}

fn main() {
    let exemplo1 = vec![10, -3, 7, 8, -1, 0, 2];
    let exemplo2 = vec![12, 2, 1, -2, 4, 5];

    println!(
        "Maior soma sem adjacentes em {:?} = {}",
        exemplo1,
        maior_soma_sem_adjacentes(&exemplo1)
    ); // 20

    println!(
        "Maior soma sem adjacentes em {:?} = {}",
        exemplo2,
        maior_soma_sem_adjacentes(&exemplo2)
    ); // 18
}
