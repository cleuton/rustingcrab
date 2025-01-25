//! ```cargo
//! [package]
//! name = "threads_arc_wrong"
//! version = "0.1.0"
//! edition = "2021"
//! [dependencies]
//! ```

use std::thread;

fn main() {
    let mut counter = 0;
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                counter = counter + 1;
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", counter);
}