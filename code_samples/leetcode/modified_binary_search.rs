#!/usr/bin/env cargo-script

//! ```cargo
//! [package]
//! name = "modified_binary_search"
//! version = "0.1.0"
//! edition = "2021"
//!
//! [dependencies]
//! ```

fn mediana(slice1: &[i32], slice2: &[i32]) -> f64 {
    let tamanho1 = slice1.len() as isize;
    let tamanho2 = slice2.len() as isize;
    let total = tamanho1 + tamanho2;
    let metade = (total + 1) / 2;

    // índices de partição em slice1 e slice2, podem ficar negativos ou além do fim
    let mut fa = (tamanho1 + 1) / 2 - 1;
    let mut fb = metade - (fa + 1) - 1;

    loop {
        // pega o valor ou +/− infinito conforme out of bounds
        let p1  = if fa  < 0          { i32::MIN } else { slice1[fa  as usize] };
        let p1o = if fa+1 >= tamanho1    { i32::MAX } else { slice1[(fa+1) as usize] };
        let p2  = if fb  < 0          { i32::MIN } else { slice2[fb  as usize] };
        let p2o = if fb+1 >= tamanho2    { i32::MAX } else { slice2[(fb+1) as usize] };

        // partição correta?
        if p1 <= p2o && p2 <= p1o {
            return if total % 2 == 0 {
                (p1.max(p2) as f64 + p1o.min(p2o) as f64) / 2.0
            } else {
                p1.max(p2) as f64
            };
        }

        // ajusta fa/fb binariamente
        if p1 <= p2o {
            fa += 1;
            fb -= 1;
        } else {
            fa -= 1;
            fb += 1;
        }
    }
}

fn main() {
    println!("{}", mediana(&[],      &[1]));                       // 1.0
    println!("{}", mediana(&[2],     &[]));                        // 2.0
    println!("{}", mediana(&[0, 0],  &[0, 0]));                    // 0.0
    println!("{}", mediana(&[1,3,7,9], &[1,5,9,20]));              // 6.0
    println!("{}", mediana(&[1,3,7,9], &[1,9,20]));                // 7.0
    println!("{}", mediana(&[1,1,1,2], &[3,4,8,9,10,10]));         // 3.5
    println!("{}", mediana(&[1,2,3,3,5], &[8,11,13]));             // 4.0
}
