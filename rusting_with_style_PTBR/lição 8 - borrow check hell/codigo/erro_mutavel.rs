fn main() {
    // Atenção a essa linha: 
    let mut s = String::from("Olá"); 

    // Empréstimos imutáveis (ok)
    let r1 = &s;

    // Deveria dar erro? Não. Você não está alterando "s". 
    println!("Original {}", s); // Não dá erro
    let r2 = &s; 
    println!("Imutáveis: {} e {}", r1, r2); // Funciona

    // Agora tentamos criar um empréstimo mutável...
    let r3 = &mut s; 
    // E AO MESMO TEMPO usar `s` (ou mesmo as referências imutáveis) na mesma "janela" de vida.
    println!("Tentando usar s e r3: {} e {}", s, r3);
  }