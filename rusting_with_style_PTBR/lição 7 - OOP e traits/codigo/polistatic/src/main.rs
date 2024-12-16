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

fn fazer_barulho<T: Animal>(animal: &T) {
    println!("Na função: {} diz {}", animal.nome(), animal.fazer_barulho());
}

fn main() {

    let cachorro = Cachorro {
        nome: String::from("Rex"),
    };  
    let gato = Gato {
        nome: String::from("Miau"),
    };
    
    println!("{} diz {}", cachorro.nome(), cachorro.fazer_barulho());
    println!("{} diz {}", gato.nome(), gato.fazer_barulho());

    fazer_barulho(&cachorro);
    fazer_barulho(&gato);
}
