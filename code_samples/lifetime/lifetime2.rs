#!/usr/bin/env cargo-script

//! ```cargo
//! [package]
//! name = "lifetime2"
//! version = "0.1.0"
//! edition = "2021"
//!
//! [dependencies]
//! ```

#[derive(Debug)]
struct Carro<'a> {
    consumo: i32,
    cor: &'a str,
    vel_maxima: i32,
}

impl<'a> Carro<'a> {
    fn alterar_consumo(&mut self, consumo: i32) {
        self.consumo = consumo;
    }
    fn alterar_cor(&mut self, cor: &'a str) {
        self.cor = cor;
    }
    fn alterar_vel_maxima(&mut self, vel_maxima: i32) {
        self.vel_maxima = vel_maxima;
    }
}

fn main() {
    let mut Carro = Carro{consumo: 10, cor: "Branco", vel_maxima: 120};
    Carro.alterar_consumo(20);
    let cor = String::from("Vermelho");
    Carro.alterar_cor(cor.as_str());
    Carro.alterar_vel_maxima(150);
    println!("{:?}",Carro);
}