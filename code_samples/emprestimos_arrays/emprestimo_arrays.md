![](../../rusting-crab-logo.png)

# Arrays, empréstimos, movimentações e cópia

É difícil entender essa questão no início e mesmo as **IAs** erram ao analisarem código **Rust** sob estes (e outros) aspectos. 

## Trait Copy

Em Rust, variáveis de “tipos triviais” de tamanho fixo e sem gerenciamento próprio de recursos, implementam o trait **Copy** automaticamente. Isso inclui, por exemplo, todos os tipos numéricos (i32, u64, f64, char, bool), ponteiros crus (\*const T, \*mut T), referências imutáveis (\&T), arrays de tamanho fixo (\[T; N]) sempre que o tipo de dados dos elementos do array implementem o trait Copy, tuplas (T1, T2, …, TN) desde que todos os tipos implementem Copy, e enums ou structs cujos campos também implementem Copy.

O efeito prático é que, em vez de mover o valor numa atribuição, passagem de parâmetro ou retorno de função, o compilador gera uma cópia bit-a-bit “de graça” e mantém a origem intacta. Você não vai esbarrar naquele famoso erro “value moved here, use after move” para esses tipos simples, e também não precisa invocar clone() o tempo todo. 

## Arrays de tamanho fixo

Em Rust os Arrays são de tamanho fixo e são declarados como: [T;n], onde "T" é o tipo de dados dos elementos e "n" é o tamanho do array.

Já os **slices** são diferentes...

## Slices

Slices em Rust são "visões" de uma sequência contígua de elementos cujo tamanho só fica definido em tempo de execução. Diferente do array, que é `[T; N]` e carrega o `N` no tipo (sempre fixo), o slice é um **DST** (dynamic-sized type): Você nunca vai ter um ` [T]` puro, sempre o acessará através de um ponteiro que carrega junto o comprimento, como: `&[T]` (slice imutável) ou `&mut [T]` (slice mutável).

Na prática, isso significa que quando você faz algo como:

```rust
let arr = [10, 20, 30, 40];
let fatia: &[i32] = &arr[1..3]; // fatia aponta para os elementos 20 e 30, e sabe que len() == 2
```

o compilador monta um *fat pointer* (dois elementos: um para o endereço base e outro para o comprimento) e você passa esse *fat pointer* por aí. Slices não têm ownership dos dados, são apenas “refências com tamanho”.

Por serem referências, `&[T]` e `&mut [T]` são `Copy` (você copia o fat pointer, não o conteúdo), mas o próprio tipo ` [T]` (sem ponteiro) não faz sentido ser instanciado isoladamente. Quando você precisa de uma coleção cujo tamanho varia, use `Vec<T>` (que aloca no heap) e ele automaticamente se derreferencia para `&[T]` quando você chamar métodos que aceitam slices. Ou, se quiser a fatia proprietária mas sem crescer, há `Box<[T]>`, que também guarda o comprimento junto ao ponteiro.

## Brincando com arrays

A confusão começa quando você tem um código assim: 

```rust

fn aloca(v: [u8;5])  {
    println!("Tamanho: {:?}", v.len());
}

fn mostra(v: [u8;5]) {
    for i in v.iter() {
        println!("Valor: {}", i);
    }
}

fn main() {
    let mut a = [1,2,3,4,5];
    let b = &a;
    a[0] = 10; 
    aloca(a);
    mostra(a);
}
```

O código acima rodaria? Ou daria erro de compilação? Analise bem e me diga o que pensa...

Primeiro, vejamos o que o código faz e como se comportam as variáveis: 

```rust
fn aloca(v: [u8;5])  {
    // "v" é uma cópia do array original.
    // Arrays de tamanho fixo implementam Copy, 
    // desde que seus elementos também implementem Copy.
    println!("Tamanho: {:?}", v.len());
}

fn mostra(v: [u8;5]) {
    // O mesmo se aplica aqui
    for i in v.iter() {
        println!("Valor: {}", i);
    }
}

fn main() {
    // Array mutável de tamanho fixo
    let mut a = [1,2,3,4,5];
    // "b" é uma referência imutável ao array "a".
    let b = &a;
    // Alteramos um dos elementos de "a"
    a[0] = 10; 
    // "aloca" recebe uma cópia do array "a"
    aloca(a);
    // "mostra" recebe uma cópia do array "a"
    mostra(a);
    // "b" ainda aponta para o array original, que foi modificado.
}
``` 

Se você disse que daria algum erro de compilação, errou. O código funciona sem problemas. Todos ficariam tentados em dizer que daria erro de compilação, geralmente em algumas linhas: 

- `a[0] = 10;`: Aqui, pois havia um "empréstimo" imutável à variável "b" e você alterou o array.
- `aloca(a)`: Pela mesma razão, pois o array estaria sendo "movido" para a função "aloca".
- `mostra(a)`: Idem.

Mas você deve se lembrar que arrays de tamanho fixo implementam o trait **Copy** se o tipo de dados dos seus elementos também o implementar. Nesse caso: 

- `aloca(a)`: É passada uma cópia do array para a função.
- `mostra(a)`: Idem.

Ok, mas e a questão do empréstimo imutável para "b"? O compilador deveria reclamar, correto, afinal de contas, a variável "b" ainda estaria dentro do escopo. Porém, ele é inteligente o suficiente para saber que você não usou mais a variável "b", portanto, a alteração não vai causar problema de memória. 

> Quer ver dar erro? 

Adicione essa linha ao final da função `main()`: 

```rust
fn main() {
    let mut a = [1,2,3,4,5];
    let b = &a;
    a[0] = 10; 
    aloca(a);
    mostra(a);

    println!("Valor de b: {:?}", b); // <--- Xabú aqui
}
```

O compilador vai reclamar com uma mensagem parecida com essa: 

```shell
error[E0506]: cannot assign to `a[_]` because it is borrowed
  --> teste1.rs:22:5
   |
20 |     let b = &a;
   |             -- `a[_]` is borrowed here
21 |     // Alteramos um dos elementos de "a"
22 |     a[0] = 10; 
   |     ^^^^^^^^^ `a[_]` is assigned to here but it was already borrowed
...
32 |     println!("Valor de b: {:?}", b);
   |                                  - borrow later used here
```

O valor do array foi "emprestado" quando criamos uma referência imutável para ele na variável "b", e esse empréstimo foi utilizado depois de tentarmos alterar o array.

## E se fosse um array de String?

Aí é completamente diferente!

> Arrays cujos elementos são de tipos complexos, que gerenciam memória, não implementam o trait **Copy**. 

O que isso significa? Que sempre que você atribuir o array a uma variável, ou usar como argumento de função, você estará movendo o array. Se não quiser fazer isso, tem que implementar o trait **Clone**. Se o array for de **String**, ele implementa o método trait e tem o método `clone()`. É só invocar o método para criar um clone do array e evitar a movimentação.

Vejamos esse código, baseado naquele anterior, só trocando o tipo dos elementos: 

```rust 

fn aloca(v: [String;5])  {
    println!("Tamanho: {:?}", v.len());
}

fn mostra(v: [String;5]) {
    // O mesmo se aplica aqui
    for i in v.iter() {
        println!("Valor: {}", i);
    }
}

fn main() {
    let mut a: [String; 5] = ["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(), "e".to_string()];
    let b = &a;
    a[0] = "z".to_string(); 
    aloca(a);
    mostra(a);

    //println!("Valor de b: {:?}", b);
}
``` 

Este código daria erro de compilação? Onde? Sim, daria erro de compilação exatamente na linha: `mostra(a);`, pois o array foi MOVIDO para a função `aloca()` invocada antes dele. Como evitar isso? Uma das maneiras seria usar `clone()` antes de passar o array como argumento: 

```rust
    aloca(a.clone());
    mostra(a.clone());
```

Teoricamente, só precisaria clonar na chamada de `aloca()`.

Outra possibilidade (mais profissional) seria aceitar referências de arrays como argumentos das funções: 

```rust
fn aloca(v: &[String])  {
    ...
}

fn mostra(v: &[String]) {
    // O mesmo se aplica aqui
    ...
}

fn main() {
    ...
    aloca(&a);
    mostra(&a);
    ...
}
```

Mesmo assim, se descomentarmos aquele último `println!` no final da função `main()`, tomaríamos esse erro: 

```shell

error[E0506]: cannot assign to `a[_]` because it is borrowed
  --> teste2.rs:20:5
   |
17 |     let b = &a;
   |             -- `a[_]` is borrowed here
...
20 |     a[0] = "z".to_string(); 
   |     ^^^^ `a[_]` is assigned to here but it was already borrowed
...
32 |     println!("Valor de b: {:?}", b);
   |                                  - borrow later used here
```

Pois criamos um "empréstimo" imutável ao atribuir uma referência do array à variável "b" e, enquanto "b" estiver no escopo, não podemos alterar o array.