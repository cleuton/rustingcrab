trait Animal {
    fn nome(&self) -> String;
    fn fazer_barulho(&self) -> String;
}

struct Cachorro {
    nome: String,
}

impl Animal for Cachorro {
    fn nome(&self) -> String {
        self.nome.clone()
    }
    fn fazer_barulho(&self) -> String {
        String::from("Au Au")
    }
}

struct Gato {
    nome: String,
}

impl Animal for Gato {
    fn nome(&self) -> String {
        self.nome.clone()
    }
    fn fazer_barulho(&self) -> String {
        String::from("Miau")
    }
}

fn fazer_barulho(animal: &dyn Animal) {
    println!("Na função: {} diz {}", animal.nome(), animal.fazer_barulho());
}

fn main() {

    let animal1: &dyn Animal = &Cachorro {
        nome: String::from("Rex"),
    };
    let animal2: &dyn Animal = &Gato {
        nome: String::from("Sapeca"),
    };
    
    println!("{} diz {}", animal1.nome(), animal1.fazer_barulho());
    println!("{} diz {}", animal2.nome(), animal2.fazer_barulho());

    fazer_barulho(animal1);
    fazer_barulho(animal2);
}
