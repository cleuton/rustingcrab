#!/usr/bin/env cargo-script

//! ```cargo
//! [package]
//! name = "subarray_sum"
//! version = "0.1.0"
//! edition = "2021"
//!
//! [dependencies]
//! ```

use std::collections::HashMap;

fn subarray_sum(nums: &[i32], k: i32) -> i32 {
    let mut soma_prefix = 0;
    let mut freq = HashMap::<i32, usize>::new();
    let mut resposta = 0;
    freq.insert(0, 1); // Para lidar com subarrays que começam do índice 0

    for &valor in nums {
        soma_prefix += valor; 
        let comp = soma_prefix - k;
        resposta += freq.get(&(soma_prefix - k)).unwrap_or(&0);
        *freq.entry(soma_prefix).or_insert(0) += 1
    }
    resposta as i32
}

fn main() {
    let nums = vec![1, 1, 1];
    let k = 2;
    let resultado = subarray_sum(&nums, k);
    println!("{}", resultado); // 2

    let nums2 = vec![1, 2, 3];
    let k2 = 3;
    let resultado2 = subarray_sum(&nums2, k2);
    println!("{}", resultado2); // 2

    let nums3 = vec![3, 4, 7, 2, -3, 1, 4, 2];
    let resultado3 = subarray_sum(&nums3, 7);
    println!("{}", resultado3); // 4
}
