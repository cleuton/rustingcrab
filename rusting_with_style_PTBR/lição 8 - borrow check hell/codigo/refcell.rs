use std::cell::RefCell;

fn main() {
    let dados = RefCell::new(5);
    
    // Empréstimo mutável
    *dados.borrow_mut() += 1;
    
    // Temos borrow() e borrow_mut(): 
    println!("Dados: {}", dados.borrow()); 
}