include!(concat!(env!("OUT_DIR"), "/saudacao.rs"));

fn main() {
    println!("{}", saudar());
}