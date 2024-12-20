use std::rc::Rc;

fn main() {
    let dados = Rc::new(5);
    let referencia1 = Rc::clone(&dados);
    let referencia2 = Rc::clone(&dados);

    println!("Dados: {}", dados);
    println!("Referência 1: {}", referencia1);
    println!("Referência 2: {}", referencia2);
}