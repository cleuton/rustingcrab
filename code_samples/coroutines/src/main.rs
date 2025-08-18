#![feature(coroutines, coroutine_trait, yield_expr, stmt_expr_attributes)]

use std::ops::{Coroutine, CoroutineState};
use std::pin::Pin;

fn main() {
    let limite = 100;

    // coroutine que gera primos até `limite`
    let mut primos = #[coroutine] move || {
        if limite < 2 {
            return ();
        }

        // crivo clássico
        let mut eh_primo = vec![true; limite + 1];
        eh_primo[0] = false;
        eh_primo[1] = false;

        let mut i = 2usize;
        while i * i <= limite {
            if eh_primo[i] {
                let mut m = i * i;
                while m <= limite {
                    eh_primo[m] = false;
                    m += i;
                }
            }
            i += 1;
        }

        // produz os primos com `yield`
        for p in 2..=limite {
            if eh_primo[p] {
                yield p; // suspende e entrega `p`
            }
        }

        // retorno final (sem valor extra)
        ()
    };

    // consome a coroutine até completar
    print!("Primos até {limite}:");
    loop {
        match Pin::new(&mut primos).resume(()) {
            CoroutineState::Yielded(p) => print!(" {p}"),
            CoroutineState::Complete(()) => break,
        }
    }
    println!();
}
