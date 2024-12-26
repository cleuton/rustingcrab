use std::rc::Rc;

fn main() {
    let dado = Rc::new(String::from("Olá, mundo!"));
    let r1 = Rc::clone(&dado); 
    let r2 = Rc::clone(&dado);

    println!("dado: {}", dado);
    println!("r1: {}", r1);
    println!("r2: {}", r2);
    println!("Número de donos: {}", Rc::strong_count(&dado));
}