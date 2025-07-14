#!/usr/bin/env cargo-script

//! ```cargo
//! [package]
//! name = "bthree_transversal"
//! version = "0.1.0"
//! edition = "2021"
//!
//! [dependencies]
//! ```

use std::collections::BTreeMap;

enum Arvore {
    Vazio,
    No(i32, Box<Arvore>, Box<Arvore>),
}

fn construir_arvore(anos: &[i32]) -> Arvore {
    if anos.is_empty() {
        Arvore::Vazio
    } else {
        let meio = anos.len() / 2;
        Arvore::No(
            anos[meio],
            Box::new(construir_arvore(&anos[..meio])),
            Box::new(construir_arvore(&anos[meio + 1..])),
        )
    }
}

fn pesquisar(arvore: &Arvore, ano: i32) -> bool {
    match arvore {
        Arvore::Vazio => false,
        Arvore::No(valor, esq, dir) if *valor == ano => true,
        Arvore::No(valor, esq, dir) if ano < *valor => pesquisar(esq, ano),
        Arvore::No(_, _, dir) => pesquisar(dir, ano),
    }
}

fn ancestral_comum(arvore: &Arvore, a: i32, b: i32) -> Option<i32> {
    let (menor, maior) = if a < b { (a, b) } else { (b, a) };
    match arvore {
        Arvore::Vazio => None,
        Arvore::No(valor, esq, dir) if maior < *valor => ancestral_comum(esq, menor, maior),
        Arvore::No(valor, esq, dir) if menor > *valor => ancestral_comum(dir, menor, maior),
        Arvore::No(valor, _, _) => Some(*valor),
    }
}

fn pre_ordem(arvore: &Arvore, saida: &mut Vec<i32>) {
    if let Arvore::No(valor, esq, dir) = arvore {
        saida.push(*valor);
        pre_ordem(esq, saida);
        pre_ordem(dir, saida);
    }
}

fn em_ordem(arvore: &Arvore, saida: &mut Vec<i32>) {
    if let Arvore::No(valor, esq, dir) = arvore {
        em_ordem(esq, saida);
        saida.push(*valor);
        em_ordem(dir, saida);
    }
}

fn pos_ordem(arvore: &Arvore, saida: &mut Vec<i32>) {
    if let Arvore::No(valor, esq, dir) = arvore {
        pos_ordem(esq, saida);
        pos_ordem(dir, saida);
        saida.push(*valor);
    }
}

fn main() {
    let mut mapa_eventos = BTreeMap::new();
    mapa_eventos.insert(1958, "Brasil é campeão mundial de futebol na Suécia");
    mapa_eventos.insert(1962, "Eleição do Papa Paulo VI");
    mapa_eventos.insert(1954, "Primeiro transplante de órgão - Rim");
    mapa_eventos.insert(1962, "Crise dos mísseis em Cuba");
    mapa_eventos.insert(1951, "Getúlio Vargas assume seu segundo mandato");
    mapa_eventos.insert(1955, "Primeira vacina contra poliomielite");
    mapa_eventos.insert(1960, "Inauguração de Brasília");
    mapa_eventos.insert(1949, "Criação da OTAN");
    mapa_eventos.insert(1959, "Criação da SUDENE");
    mapa_eventos.insert(1952, "Primeiro teste de bomba de hidrogênio");
    mapa_eventos.insert(1961, "Iuri Gagarin primeiro humano no espaço");
    mapa_eventos.insert(1958, "Revolução Cubana");
    mapa_eventos.insert(1963, "Estreia do primeiro álbum dos Beatles");
    mapa_eventos.insert(1950, "Uruguai campeão mundial no Maracanã");
    mapa_eventos.insert(1957, "Lançamento do Sputnik");
    mapa_eventos.insert(1956, "Elvis Presley lança primeiro álbum");
    mapa_eventos.insert(1953, "Getúlio Vargas cria a Petrobras");

    let anos: Vec<i32> = mapa_eventos.keys().cloned().collect();
    let arvore = construir_arvore(&anos);

    println!("Existe 1954? {}", pesquisar(&arvore, 1954));
    println!("Existe 1952? {}", pesquisar(&arvore, 1952));

    if let Some(anc) = ancestral_comum(&arvore, 1953, 1954) {
        println!("LCA 1953–1954: {}", mapa_eventos[&anc]);
    }
    if let Some(anc) = ancestral_comum(&arvore, 1958, 1963) {
        println!("LCA 1958–1963: {}", mapa_eventos[&anc]);
    }

    let mut v = Vec::new();
    pre_ordem(&arvore, &mut v);
    println!("Pré-ordem: {:?}", v);
    v.clear();
    em_ordem(&arvore, &mut v);
    println!("Em-ordem: {:?}", v);
    v.clear();
    pos_ordem(&arvore, &mut v);
    println!("Pós-ordem: {:?}", v);
}
