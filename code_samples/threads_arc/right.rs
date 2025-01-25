//! ```cargo
//! [package]
//! name = "threads_arc_wrong"
//! version = "0.1.0"
//! edition = "2021"
//! [dependencies]
//! ```

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0)); // Arc<Mutex<i32>>
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                // Bloqueia o Mutex antes de alterar
                let mut num = counter_clone.lock().unwrap();
                *num += 1; // Incrementa o valor protegido
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Agora, vamos imprimir o valor que está no Mutex
    println!("Final counter value: {}", *counter.lock().unwrap());
}
