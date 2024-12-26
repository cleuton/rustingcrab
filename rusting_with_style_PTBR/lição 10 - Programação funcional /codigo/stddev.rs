//! ```cargo
//! [package]
//! edition = "2021"
//! ```

fn main() {
    let dados = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let n = dados.len() as f64;

    let media = dados.iter().sum::<f64>() / n;

    let desvio_padrao = (dados.iter().fold(0.0, |acc, &x| acc + (x - media).powi(2)) / (n - 1.0)).sqrt();

    println!("Desvio padrão amostral: {}", desvio_padrao);
}