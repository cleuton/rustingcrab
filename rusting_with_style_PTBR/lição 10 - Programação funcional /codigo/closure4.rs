//! ```cargo
//! [package]
//! edition = "2021"
//! ```

fn teste<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(10)
}

fn outro(x: i32) -> i32 {
    x * 100
}

fn main() {
    let x = teste(|y| y + 5);
    println!("{}", x);
    let y = teste(outro);
    println!("{}", y);    
}