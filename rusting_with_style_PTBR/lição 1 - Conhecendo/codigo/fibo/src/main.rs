fn fibo(n: u32) -> u32 {
    if n < 2 {
        return n;
    }
    return fibo(n - 1) + fibo(n - 2);
}

fn main() {
    println!("Fibo de 0: {}", fibo(0));
    println!("Fibo de 1: {}", fibo(1));
    println!("Fibo de 2: {}", fibo(2));
    println!("Fibo de 3: {}", fibo(3));
    println!("Fibo de 8: {}", fibo(8));
}