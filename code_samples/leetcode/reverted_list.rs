#!/usr/bin/env cargo-script

//! ```cargo
//! [package]
//! name = "reverted_list"
//! version = "0.1.0"
//! edition = "2021"
//!
//! [dependencies]
//! ```

use std::collections::LinkedList;

fn reverse_list<T>(list: &mut LinkedList<T>) {
    let mut rev = LinkedList::new();
    // vai retirando do front da lista original e empurrando no front da nova
    while let Some(elem) = list.pop_front() {
        rev.push_front(elem);
    }
    // substitui o conteúdo da lista original pela invertida
    *list = rev;
}

fn main() {
    let mut lista = LinkedList::from([1, 2, 3, 4, 5]);
    println!("Original: {:?}", lista);
    reverse_list(&mut lista);
    println!("Invertida: {:?}", lista);
}
