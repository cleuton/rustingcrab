fn main() {
    let numero = 5;

    match numero {
        x if x % 2 == 0 => println!("O número {} é par", x),
        x => println!("O número {} é ímpar", x),
    }
}