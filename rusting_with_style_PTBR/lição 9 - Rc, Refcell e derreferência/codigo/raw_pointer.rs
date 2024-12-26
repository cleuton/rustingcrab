fn main() {
    let p: *mut String;
    {
        let nome = String::from("Fulano");
        p = &nome as *const String as *mut String; 
    } 
    // `nome` saiu de escopo, a memória foi liberada!
    unsafe {
        // Comportamento indefinido!!!!!!!!
        println!("{}", *p); 
    }
}