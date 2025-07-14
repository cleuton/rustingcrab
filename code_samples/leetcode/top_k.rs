#!/usr/bin/env cargo-script

//! ```cargo
//! [package]
//! name = "top_k"
//! version = "0.1.0"
//! edition = "2021"
//!
//! [dependencies]
//! ```

use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;

fn top_k_frequente(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let mut contagem = HashMap::new();
    for num in nums {
        *contagem.entry(num).or_insert(0) += 1;
    }

    // min-heap pela frequência: o topo será sempre o menor (freq, num)
    let mut heap: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();
    for (&num, &qtd) in &contagem {
        heap.push(Reverse((qtd, num)));
        if heap.len() > k {
            heap.pop(); // agora remove quem tem frequência menor
        }
    }

    // extrai só os números restantes
    heap.into_iter()
        .map(|Reverse((_, num))| num)
        .collect()
}

fn main() {
    let nums = vec![1, 1, 1, 2, 2, 3];
    let k = 2;
    let res = top_k_frequente(nums, k);
    println!("{:?}", res); // saída: [1, 2] (ou [2, 1])
}
