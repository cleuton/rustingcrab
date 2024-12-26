use std::sync::Arc;
use std::thread;

fn main() {
    // Cria um Arc para compartilhar o valor entre threads
    let dado = Arc::new(String::from("Olá, mundo!"));

    // Clonamos o Arc para cada thread (isso não copia o dado, 
    // apenas incrementa o contador de referência atômico)
    let r1 = Arc::clone(&dado);
    let r2 = Arc::clone(&dado);

    // Criamos duas threads, cada uma usando seu clone do Arc
    let t1 = thread::spawn(move || {
        println!("Thread 1: {}", r1);
    });

    let t2 = thread::spawn(move || {
        println!("Thread 2: {}", r2);
    });

    // Esperamos as threads terminarem
    t1.join().unwrap();
    t2.join().unwrap();

    // Mostra que o valor original ainda existe aqui
    println!("Main thread: {}", dado);

    // Quando nenhuma referência (Arc) existir mais, 
    // o valor será desalocado automaticamente.
}