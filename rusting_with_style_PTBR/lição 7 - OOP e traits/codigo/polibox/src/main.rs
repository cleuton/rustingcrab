trait Animal {
    fn nome(&self) -> &str;
    fn fazer_barulho(&self) -> &str;
}

struct Cachorro {
    nome: String,
}

impl Animal for Cachorro {
    fn nome(&self) -> &str {
        &self.nome
    }
    fn fazer_barulho(&self) -> &str {
        "Au Au"
    }
}

struct Gato {
    nome: String,
}

impl Animal for Gato {
    fn nome(&self) -> &str {
        &self.nome
    }
    fn fazer_barulho(&self) -> &str {
        "Miau"
    }
}

fn main() {
    // Criação de uma coleção heterogênea de objetos implementando Animal
    let animais: Vec<Box<dyn Animal>> = vec![
        Box::new(Cachorro { nome: String::from("Rex") }),
        Box::new(Gato { nome: String::from("Felix") }),
    ];

    // Interagindo com cada objeto da coleção de forma polimórfica
    for animal in animais.iter() {
        println!("{} diz {}", animal.nome(), animal.fazer_barulho());
    }
}
