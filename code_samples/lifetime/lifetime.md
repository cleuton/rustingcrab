![](../../rusting-crab-logo.png)

# Entendendo lifetime

[*Cleuton Sampaio*](https://linkedin.com/in/cleutonsampaio). Me siga. 

[**RustingCrab Repo**](https://rustingcrab.com). Marque com **star**!

Lifetime é um conceito estranho para quem está aprendendo **Rust**. Vamos tentar explicar da maneira mais simples e sem ambiguidade possível. De acordo com o site "Rust by Example": 

> *Lifetime* é uma construção que o compilador (ou, mais especificamente, seu *Borrow checker*) usa para garantir que todos os empréstimos sejam válidos. Especificamente, o tempo de vida de uma variável começa quando ela é criada e termina quando ela é destruída.

Se você cria uma variável não-primitiva (aquela que armazena o dado fora da *stack*), você precisaria gerenciar essa memória, caso contrário, poderia incorrer em *memory leak* ou mesmo *dangling pointer* (quando o ponteiro aponta uma memória já liberada). Algumas linguagens de programação utilizam *Garbage Collector* para isso, enquanto outras apenas usam *smart pointers* (contagem de referências, por exemplo).

Em Rust, a abordagem é diferente. O compilador se encarrega de verificar se você pode fazer alguma bobagem e nem compila seu código. Isso é segurança de memória em tempo de compilação. Daí os conceitos de *ownership* (propriedade) e *borrow* (empréstimo). Mas há uma situação curiosa que fica "de fora": E se você passar uma referência para uma função ou instância de *struct*? Como fica isso? E se a instância ou a função durarem mais do que a variável cuja referência foi passada? Aí é que entra o **lifetime**.

## Mas o que é lifetime?

De acordo com o livro "Rust programming language": 

> **Lifetimes** são outro tipo de *genérico* que já usamos. Em vez de garantir que um tipo tenha o comportamento que desejamos, os *lifetimes* garantem que as referências sejam válidas pelo tempo que precisarmos.

Então vemos que *lifetime* é uma anotação de tipo, utilizada quando declaramos funções genéricas. Mas tem mais:

> **Lifetime** em Rust é um label que indica o tempo de vida de uma referência para garantir que ela sempre aponte para dados válidos.

Você precisa indicar o *lifetime* de uma referência ou função sempre que o compilador não conseguir inferir automaticamente qual referência está sendo usada ou por quanto tempo ela permanece válida, como em funções que retornam referências, estruturas que armazenam referências ou situações com múltiplas referências com durações diferentes.

Em Rust você só precisa anotar lifetimes quando o compilador não consegue inferir por quanto tempo suas referências permanecem válidas. Isso ocorre em quatro situações principais:

1. Funções que retornam referências.
2. Structs que armazenam referências em campos.
3. Funções com vários parâmetros de referência que têm lifetimes distintos.
4. Parâmetros genéricos ou trait-objects anotados com lifetimes (mesmo que retornem `()`).

Basta declarar algo como `fn nome<'a>(…)` ou `struct Tipo<'a> { … }` e ligar o `'a` aos parâmetros (e ao retorno, quando houver) para deixar claro ao compilador qual referência vive mais tempo.

## Exemplo

Vamos ver um exemplo muito simples mesmo. No repo há um arquivo `lifetime2.rs` que vou reproduzir aqui: 

```Rust
#!/usr/bin/env cargo-script

//! ```cargo
//! [package]
//! name = "lifetime2"
//! version = "0.1.0"
//! edition = "2021"
//!
//! [dependencies]
//! ```

#[derive(Debug)]
struct Carro<'a> {
    consumo: i32,
    cor: &'a str,
    vel_maxima: i32,
}

impl<'a> Carro<'a> {
    fn alterar_consumo(&mut self, consumo: i32) {
        self.consumo = consumo;
    }
    fn alterar_cor(&mut self, cor: &'a str) {
        self.cor = cor;
    }
    fn alterar_vel_maxima(&mut self, vel_maxima: i32) {
        self.vel_maxima = vel_maxima;
    }
}

fn main() {
    let mut Carro = Carro{consumo: 10, cor: "Branco", vel_maxima: 120};
    Carro.alterar_consumo(20);
    let cor = String::from("Vermelho");
    Carro.alterar_cor(cor.as_str());
    Carro.alterar_vel_maxima(150);
    println!("{:?}",Carro);
}
```

Ok... Estranho pacas! Vamos devagar... 

### `struct Carro<'a>`

Precisamos declarar o `'a` porque o campo `cor` é uma referência (`&str`) e, ao contrário de funções, não existe elisão de lifetimes para structs. Sempre que um tipo armazena uma referência, você tem que dizer ao compilador quanto tempo essa referência vai viver. O parâmetro genérico `<'a>` faz exatamente isso: vincula o tempo de vida de cada `Carro<'a>` ao da string apontada em `cor`, garantindo que você não acabe com um `Carro` referenciando memória já liberada.

Em outras palavras, sem `<'a>` o compilador não sabe “até quando” a fatia `&str` dentro de `cor` permanece válida, e por isso ele obriga você a explicitar esse lifetime no cabeçalho do struct.

O nome do lifetime é só um rótulo arbitrário, você pode usar `'cor` em vez de `'a`. Desde que todos os locais que precisam ter o mesmo lifetime usem o mesmo `label`. Por convenção usa-se `'a`, `'b` etc., mas nomes descritivos como `'cor` também são válidos.

> O **tipo** de **Carro** agora é **Carro<'a>**!

### `impl<'a> Carro<'a>`

Porque em Rust lifetimes são parâmetros genéricos do tipo, e todo genérico (seja de tipo ou de lifetime) precisa ser declarado no `impl` antes de usar. Quando você escreve `impl<'a> Carro<'a>`, está dizendo “este bloco implementa métodos para qualquer `Carro` que tenha um lifetime `'a`”. Sem o `<'a>` no `impl`, o compilador não saberia o que esse `'a` significa, assim como você faria `impl<T> MinhaStruct<T>` para um tipo genérico `T`. Não existe elisão de lifetimes em blocos de `impl`, então é preciso explicitar o mesmo parâmetro de lifetime ali para que Rust saiba qual parâmetro genérico está em jogo.

> **Elisão**: Em Rust, elisão de lifetimes é o conjunto de regras que o compilador aplica para associar automaticamente os tempos de vida de referências em funções e métodos, permitindo omitir anotações quando há apenas uma entrada ou quando se trata de `&self`. 

### `fn alterar_cor(&mut self, cor: &'a str) {`

Anotamos o lifetime porque o parâmetro `cor` é uma fatia (`&str`) que precisa viver pelo mesmo período que o `Carro<'a>` ao qual ela será atribuída. Ao declarar

```rust
impl<'a> Carro<'a> {
    fn alterar_cor(&mut self, cor: &'a str) { … }
}
```

você está dizendo ao compilador “o novo valor de `cor` deve ter pelo menos o mesmo tempo de vida `'a` do struct.” Sem isso o Rust não conseguiria garantir que a string que você passa não seja descartada antes do `Carro` tentar usá-la, evitando referências pendentes.




