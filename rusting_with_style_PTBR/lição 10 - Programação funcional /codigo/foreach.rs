//! ```cargo
//! [package]
//! edition = "2021"
//! ```

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    numbers.iter().for_each(|&x| {
        println!("O número é: {}", x);
    });
}