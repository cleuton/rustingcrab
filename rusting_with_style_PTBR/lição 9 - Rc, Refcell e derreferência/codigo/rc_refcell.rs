use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let valor = Rc::new(RefCell::new(10));

    let v1 = Rc::clone(&valor);
    let v2 = Rc::clone(&valor);

    {
        // Borrow mutavelmente usando RefCell
        let mut ref_mut = v1.borrow_mut();
        *ref_mut += 5; // Modifica o valor de 10 para 15
    }

    // Todos veem a mesma mudança, já que compartilham o mesmo RefCell
    println!("v1: {}", v1.borrow());
    println!("v2: {}", v2.borrow());
    println!("valor: {}", valor.borrow());
}