// Usamos a instrução "use" para importar uma função ou tipo específico de outro módulo
use std::cmp::max;

/// Estrutura que representa uma pessoa
struct Pessoa {
    nome: String,
    idade: u8,
}

impl Pessoa {
    /// Método associado que cria uma nova Pessoa
    fn new(nome: String, idade: u8) -> Self {
        Pessoa { nome, idade }
    }

    /// Método que verifica se a pessoa é maior de idade
    fn maior_de_idade(&self) -> bool {
        self.idade >= 18
    }

    /// Método que retorna uma saudação personalizada
    fn saudacao(&self) -> String {
        format!("Olá, meu nome é {} e eu tenho {} anos.", self.nome, self.idade)
    }
}

/// Função simples que calcula o maior de dois números
fn maior_numero(a: i32, b: i32) -> i32 {
    max(a, b)
}

/// Função principal do programa
fn main() {
    // Criando variáveis simples
    let x = 10;
    let y = 20;

    // Chamando a função maior_numero
    let maior = maior_numero(x, y);
    println!("O maior número entre {} e {} é {}.", x, y, maior);

    // Criando uma instância de Pessoa
    let pessoa = Pessoa::new(String::from("Fulano"), 25);

    // Usando os métodos da estrutura Pessoa
    println!("{}", pessoa.saudacao());
    if pessoa.maior_de_idade() {
        println!("{} é maior de idade.", pessoa.nome);
    } else {
        println!("{} não é maior de idade.", pessoa.nome);
    }

    // Usando uma macro (println!) para imprimir uma mensagem
    println!("Este é um exemplo de programa Rust!");
}
