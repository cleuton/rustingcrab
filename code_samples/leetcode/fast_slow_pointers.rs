#!/usr/bin/env cargo-script

//! ```cargo
//! [package]
//! name = "fast_slow_pointers"
//! version = "0.1.0"
//! edition = "2021"
//!
//! [dependencies]
//! ```

use std::collections::LinkedList;

fn encontrar_meio(lista: &LinkedList<i32>) -> Option<&i32> {
    // iteradores para slow (1 passo) e fast (2 passos)
    let mut slow = lista.iter();
    let mut fast = lista.iter();
    // já pega o primeiro elemento ou retorna None se vazia
    let mut mid = slow.next()?;

    // enquanto o fast puder dar ao menos um passo...
    while fast.next().is_some() {
        // tenta dar o segundo passo em fast
        if fast.next().is_some() {
            // só então avança slow e atualiza mid
            if let Some(val) = slow.next() {
                mid = val;
            }
        } else {
            // fast não pôde dar o 2º passo: lista par, slow já está no 2º meio
            break;
        }
    }

    Some(mid)
}

fn main() {
    // lista ímpar: [1,2,3,4,5], meio → 3
    let mut l1 = LinkedList::new();
    l1.extend([1, 2, 3, 4, 5]);
    println!("{:?}", encontrar_meio(&l1)); // Some(3)

    // lista par: [10,14,29,55,65,89], meias são 29 e 55 → retorna 55
    let mut l2 = LinkedList::new();
    l2.extend([10, 14, 29, 55, 65, 89]);
    println!("{:?}", encontrar_meio(&l2)); // Some(55)

    // vazia → None
    let l3: LinkedList<i32> = LinkedList::new();
    println!("{:?}", encontrar_meio(&l3)); // None
}
