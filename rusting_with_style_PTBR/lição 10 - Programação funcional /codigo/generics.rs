//! ```cargo
//! [package]
//! edition = "2021"
//! ```

fn main() {
    // Função genérica que retorna o maior valor entre dois elementos
    fn maior<T: PartialOrd>(a: T, b: T) -> T {
        if a > b {
            a
        } else {
            b
        }
    }

    let num1 = 10;
    let num2 = 20;
    println!("O maior número é: {}", maior(num1, num2));

    let char1 = 'a';
    let char2 = 'b';
    println!("O maior caractere é: {}", maior(char1, char2));
}