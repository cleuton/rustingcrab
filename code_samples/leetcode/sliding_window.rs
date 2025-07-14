#!/usr/bin/env cargo-script

//! ```cargo
//! [package]
//! name = "sliding_window"
//! version = "0.1.0"
//! edition = "2021"
//!
//! [dependencies]
//! ```

use std::collections::HashMap;

fn encontrar(texto: &str, palavras: &[&str]) -> Option<Vec<usize>> {
    let word_len = palavras[0].len();
    let num_words = palavras.len();
    let window_len = word_len * num_words;
    let mut resultados = Vec::new();

    // mapa de frequência das palavras-alvo
    let mut target_count = HashMap::new();
    for &w in palavras {
        *target_count.entry(w).or_insert(0) += 1;
    }

    // para cada alinhamento possível dentro do tamanho da palavra
    for offset in 0..word_len {
        let mut left = offset;
        let mut right = offset;
        let mut seen = HashMap::new();
        let mut count = 0;

        // desliza a janela em blocos de word_len
        while right + word_len <= texto.len() {
            let word = &texto[right..right + word_len];
            right += word_len;

            if target_count.contains_key(word) {
                // conta esta palavra na janela vista
                *seen.entry(word).or_insert(0) += 1;
                count += 1;

                // se passou do limite de ocorrências, move left até equilibrar
                while seen[word] > target_count[word] {
                    let left_word = &texto[left..left + word_len];
                    *seen.get_mut(left_word).unwrap() -= 1;
                    left += word_len;
                    count -= 1;
                }

                // janela completa encontrada
                if count == num_words {
                    resultados.push(left);
                }
            } else {
                // palavra não esperada: reinicia a janela
                seen.clear();
                count = 0;
                left = right;
            }
        }
    }

    if resultados.is_empty() {
        None
    } else {
        Some(resultados)
    }
}

fn main() {
    let exemplos = [
        ("tensotestetestevistatestetenso", &["tenso", "teste"][..]),
        ("calpazcompazumacalcompesumapazcal", &["cal", "uma", "paz"][..]),
        ("carroratopenaratoratopanocarrocarrorato", &["carro", "pano", "rato", "carro"][..]),
    ];

    for &(texto, palavras) in &exemplos {
        match encontrar(texto, palavras) {
            Some(idxs) => println!("{:?}", idxs),
            None => println!("[]"),
        }
    }
}

