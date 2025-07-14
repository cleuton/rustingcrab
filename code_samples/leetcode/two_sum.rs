#!/usr/bin/env cargo-script

//! ```cargo
//! [package]
//! name = "two_sum"
//! version = "0.1.0"
//! edition = "2021"
//!
//! [dependencies]
//! ```

use std::collections::HashMap;

fn two_sum(nums: &[i32], target: i32) -> Option<Vec<i32>> {
    let mut visto = HashMap::<i32, usize>::new();
    let mut resposta = Vec::<i32>::new();
    for (i, &termo) in nums.iter().enumerate() {
        let comp = target - termo;
        if visto.contains_key(&comp) {
            resposta.push(visto[&comp] as i32);
            resposta.push(i as i32);
            return Some(resposta);
        }
        visto.insert(termo, i);
    }
    None
}

fn main() {
    let nums1 = vec![2, 7, 11, 15];
    let target1 = 9;
    if let Some(result) = two_sum(&nums1, target1) {
        println!("{:?}", result); // [0, 1]
    }

    let nums2 = vec![3, 2, 4];
    let target2 = 6;
    if let Some(result) = two_sum(&nums2, target2) {
        println!("{:?}", result); // [1, 2]
    }
}