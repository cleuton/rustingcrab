![](../../rusting-crab-logo.png)

# Rusting bit by bit

[**Cleuton Sampaio**](https://linkedin.com/in/cleutonsampaio) - Me siga aqui.
[**RustingCrab Repo**](https://rustingcrab.com).

## Curiosidades sobre rust

Considere o seguinte código **Rust**: 

```rust
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
```

Perguntas: 

1. Qual é a função do `use`?
2. O que é exatamente `Pessoa`? 
3. O que a declaração `impl Pessoa {}` faz? 
4. O que é `fn new ..`? 
5. O que é `fn maior_de_idade`? Qual é a diferença para `fn new`?
6. Notei que você usa `Self` (com maiúscula) e `self` (com minúscula). Qual é a diferença?
7. Por que você há momentos em que usa "::" e momentos em que usa "." para invocar métodos?
8. Qual a diferença entre `u8` e `i32`?

Essas perguntas você deve estar fazendo se é uma pessoa observadora. Vamos procurar responder aqui.

## Qual é a função do `use`? 

No código fornecido, a instrução `use std::cmp::max;` importa a função `max` do módulo `std::cmp` para o escopo atual. Isso permite que a função `max` seja utilizada diretamente como `max(a, b)` sem a necessidade de referenciá-la completamente como `std::cmp::max(a, b)` cada vez que for chamada. 

O uso da instrução `use` torna o código mais limpo e legível, evitando repetições desnecessárias de caminhos completos. Embora não seja estritamente obrigatório usar `use`—já que você poderia chamar `std::cmp::max` diretamente—a prática de importar funções ou tipos frequentemente utilizados é recomendada para simplificar e organizar melhor o código.

## O que é exatamente `Pessoa`? 

**Pessoa** é uma **struct**. Em **Rust**, uma struct é o equivalente a uma **classe**, podendo ter **propriedades** e **métodos**. Mas lembre-se que a implementação de **orientação a objetos** em Rust é simplificada, de maneira semelhante ao que ocorre em **Go** (**Golang**), portanto, **struct** não é uma **classe**, embora pareça muito com uma. 

No exemplo, cada instância da **struct** pessoa tem 2 propriedades e dois métodos: 

- Propriedade `nome`.
- Propriedade `idade`.
- Método `maior_de_idade()`.
- Método `saudacao()`.

E podemos criar instâncias da **struct Pessoa** invocando a função `new()`: 

```rust
let pessoa = Pessoa::new(String::from("Fulano"), 25);
```

## O que a declaração `impl Pessoa {}` faz?

A declaração `impl Pessoa { ... }` cria um bloco de implementação para a estrutura `Pessoa`. Dentro desse bloco, você pode definir métodos associados a `Pessoa`, sejam eles métodos que operam em uma instância específica (acessando seus campos via `&self`), métodos de classe (associados ao tipo, chamados via `Pessoa::metodo()`), ou funções auxiliares relacionadas ao tipo. 

Resumindo, o `impl` permite adicionar comportamento (funções e métodos) ao tipo definido anteriormente (`struct Pessoa`).

## O que é `fn new ..`?

`fn new(...)` é um método associado, comumente utilizado em Rust como um "construtor" de uma determinada estrutura (struct). Ele não é um construtor no sentido tradicional de linguagens orientadas a objetos, mas sim uma função estática (associada ao tipo) que cria e retorna uma nova instância do tipo definido. Esse padrão fornece uma maneira simples de inicializar os campos de uma struct e configurar qualquer lógica necessária antes de entregar a instância pronta para uso.

> Ele não é um método de uma instância de `Pessoa`, é uma função associada à **struct Pessoa**.

## O que é `fn maior_de_idade`? Qual é a diferença para `fn new`?

`fn maior_de_idade` é um método de instância da struct `Pessoa` que verifica se uma pessoa tem 18 anos ou mais, retornando `true` se for maior de idade e `false` caso contrário. Ele utiliza `&self`, o que significa que opera sobre uma instância já existente de `Pessoa`.

A diferença para `fn new` é que `new` é um método associado (estático) ao tipo `Pessoa`, usado para criar e retornar uma nova instância dessa struct. Enquanto `maior_de_idade` depende de uma instância existente para funcionar, `new` não precisa de uma instância prévia e é chamado diretamente a partir do tipo (`Pessoa::new(...)`).

## Notei que você usa `Self` (com maiúscula) e `self` (com minúscula). Qual é a diferença?

`Self` (com “S” maiúsculo) se refere ao próprio tipo dentro do bloco `impl`. Por exemplo, em `impl Pessoa`, `Self` é equivalente a `Pessoa`. Você usa `Self` em métodos associados para indicar que a função retorna ou lida com o tipo em si, sem estar ligada a nenhuma instância específica.

Já `self` (com “s” minúsculo) é usado como o primeiro parâmetro dos métodos de instância (como `fn maior_de_idade(&self)`) e se refere a uma instância específica do tipo. Dessa forma, `self` dá acesso aos campos e métodos do objeto atual em que a função está sendo chamada.

## Por que você há momentos em que usa "::" e momentos em que usa "." para invocar métodos?

Quando um método ou função é chamado utilizando `::`, estamos acessando métodos associados ao tipo, também chamados de métodos estáticos. Esses métodos não precisam de uma instância para serem invocados; eles se referem diretamente à definição do tipo. Por exemplo, `Pessoa::new(...)` cria uma nova instância da struct `Pessoa` sem precisar de uma instância prévia.

Já quando usamos o operador `.`, estamos chamando um método em uma instância já existente de um tipo. Nesse caso, o método recebe automaticamente uma referência para o objeto atual através do parâmetro `self` (normalmente `&self`), permitindo acessar seus campos e comportamentos particulares. Por exemplo, `pessoa.maior_de_idade()` chama o método `maior_de_idade` em uma instância específica de `Pessoa`.

## Qual a diferença entre `u8` e `i32`?

`u8` é um tipo de dado inteiro **sem sinal** com tamanho de 8 bits, representando valores de 0 a 255. Já `i32` é um tipo de dado inteiro **com sinal** de 32 bits, capaz de representar valores tanto negativos quanto positivos, indo de -2.147.483.648 até 2.147.483.647. Em resumo, `u8` só aceita números não negativos e ocupa menos espaço, enquanto `i32` pode lidar com uma faixa muito maior de valores, incluindo números negativos.