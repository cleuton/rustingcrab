use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let dados = Rc::new(RefCell::new(5));
    
    let referencia1 = Rc::clone(&dados);
    let referencia2 = Rc::clone(&dados);
    
    // Modificando através de uma referência
    *referencia1.borrow_mut() += 10;
    
    println!("Dados via referencia1: {}", referencia1.borrow());
    println!("Dados via referencia2: {}", referencia2.borrow());
    println!("Dados originais: {}", dados.borrow());
}