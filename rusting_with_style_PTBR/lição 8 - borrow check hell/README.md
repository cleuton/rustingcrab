# Rusting with style - Curso básico de linguagem Rust

<img src="../../rusting-crab-logo.png" alt="Descrição da imagem" style="height: 200px;">

[**Cleuton Sampaio**](https://linkedin.com/in/cleutonsampaio)

[**Veja no GitHub**](https://github.com/cleuton/rustingcrab)

[**Menu do curso**](../)

[**VÍDEO DESTA AULA**]()


# Borrow check hell

"Borrow Check Hell" é uma expressão utilizada pela comunidade Rust para descrever a frustração que muitos desenvolvedores enfrentam ao lidar com as rigorosas regras do *borrow checker* do Rust. O *borrow checker* é uma parte fundamental do compilador Rust que assegura a segurança de memória, garantindo que não haja referências inválidas ou concorrências inseguras no código. No entanto, essas mesmas regras podem, às vezes, tornar o desenvolvimento mais desafiador, especialmente para quem está começando ou para projetos mais complexos.

Essa "agonia" ocorre principalmente quando o *borrow checker* impede que certas operações sejam realizadas porque elas violam as regras de propriedade e empréstimo do Rust. Por exemplo, tentar modificar uma estrutura de dados enquanto ainda existem referências imutáveis a ela pode resultar em erros que, embora importantes para a segurança, podem ser difíceis de resolver inicialmente. Esses erros costumam ser acompanhados de mensagens de compilação detalhadas, mas que nem sempre são fáceis de entender, aumentando a sensação de "cegueira" para o desenvolvedor.

Para mitigar o "Borrow Check Hell", é essencial entender profundamente como o sistema de propriedade e empréstimo do Rust funciona. Reestruturar o código para reduzir a complexidade das referências, limitar o escopo das variáveis e utilizar ferramentas como `RefCell` ou `Rc` quando apropriado são estratégias eficazes. Além disso, com a prática e a experiência, os desenvolvedores aprendem a antecipar e evitar esses conflitos, tornando o processo de desenvolvimento mais fluido e menos frustrante. Em suma, embora o "Borrow Check Hell" represente um desafio inicial, ele é um reflexo das poderosas garantias de segurança que Rust oferece, contribuindo para a criação de softwares mais robustos e confiáveis.

Vamos explicar os conceitos de **movimento**, **propriedade** e **empréstimo** em Rust de forma simples e resumida, além de abordar como eles se aplicam a tipos primitivos.

## 1. Propriedade (Ownership)

- **O que é?**
  - Em Rust, cada valor tem **uma única variável** que "possui" esse valor. Essa variável é responsável por gerenciar a memória desse valor.

- **Por que é importante?**
  - Garante segurança na gestão de memória sem a necessidade de um coletor de lixo (garbage collector).

## 2. Movimento (Move)

- **O que é?**
  - Quando você atribui um valor de uma variável para outra, a **propriedade** desse valor é **transferida** para a nova variável. A variável original **não pode mais** ser usada.

- **Exemplo:**
  ```rust
  let s1 = String::from("Olá");
  let s2 = s1; // Movimento: s1 deixa de ser válido
  // println!("{}", s1); // Erro! s1 não é mais válido
  println!("{}", s2); // Funciona
  ```

- **Quando ocorre?**
  - Com tipos que **não implementam** o trait `Copy`, como `String` ou `Vec<T>`.

## 3. Empréstimo (Borrowing)

- **O que é?**
  - Permite que múltiplas partes do código acessem um valor sem **tomar a propriedade** dele. Isso é feito através de **referências**.

- **Tipos de Empréstimos:**
  - **Imutável (`&T`)**: Várias referências podem existir simultaneamente.
  - **Mutável (`&mut T`)**: Apenas uma referência mutável pode existir por vez.

- **Exemplo:**
  ```rust
  let s = String::from("Olá");
  let r1 = &s; // Empréstimo imutável
  let r2 = &s; // Outro empréstimo imutável
  println!("{} e {}", r1, r2); // Funciona

  let mut s_mut = String::from("Olá");
  let r_mut = &mut s_mut; // Empréstimo mutável
  r_mut.push_str(" Mundo!");
  println!("{}", r_mut); // Funciona
  ```

## 4. Tipos Primitivos e `Copy`

- **O que são tipos primitivos?**
  - Tipos básicos como `i32`, `f64`, `bool`, `char`, etc.

- **Trait `Copy`:**
  - Tipos que implementam o trait `Copy` são **copiados** em vez de movidos. Isso significa que, ao atribuir ou passar esses valores, uma **cópia** é feita e ambas as variáveis continuam válidas.

- **Por que tipos primitivos implementam `Copy`?**
  - Eles são pequenos e simples de copiar, não requerem gerenciamento complexo de memória.

- **Exemplo com `Copy`:**
  ```rust
  let x = 10;
  let y = x; // Cópia: x ainda é válido
  println!("x: {}, y: {}", x, y); // Funciona
  ```


- **Propriedade:** Cada valor tem uma única variável que o possui.
- **Movimento:** Transferência de propriedade para outra variável; a original deixa de ser válida. Ocorre com tipos que **não implementam** `Copy`.
- **Empréstimo:** Acesso a um valor sem tomar a propriedade, usando referências (`&` ou `&mut`).
- **Tipos Primitivos:** Implementam `Copy`, são **copiados** em vez de movidos, permitindo que múltiplas variáveis acessem o mesmo valor sem problemas.

Entender esses conceitos é fundamental para escrever código seguro e eficiente em Rust, aproveitando ao máximo seu sistema de propriedade e gerenciamento de memória.

## Regras de propriedade e empréstimo

Claro! Vamos explorar de maneira simples as **principais regras de propriedade (ownership)** e **empréstimo (borrowing)** em Rust, acompanhadas de exemplos para facilitar a compreensão.

## **Regras de Propriedade (Ownership)**

1. **Cada valor tem um único proprietário.**
2. **Só pode haver um proprietário de cada vez.**
3. **Quando o proprietário sai de escopo, o valor é descartado (drop).**

### **1. Cada valor tem um único proprietário**

Cada valor em Rust é "possuído" por uma única variável. Essa variável é responsável por gerenciar a memória do valor.

**Exemplo:**
```rust
fn main() {
    let s = String::from("Olá, Rust!");
    // Aqui, `s` é o proprietário da String.
}
```

### **2. Só pode haver um proprietário por vez**

Quando você atribui um valor de uma variável para outra, a propriedade é transferida (movida) para a nova variável. A variável original deixa de ser válida.

**Exemplo:**
```rust
fn main() {
    let s1 = String::from("Olá");
    let s2 = s1; // Movimento: s1 deixa de ser válido e s2 passa a ser o proprietário.

    // println!("{}", s1); // Erro! `s1` não é mais válido.
    println!("{}", s2); // Funciona, `s2` é o novo proprietário.
}
```

### **3. Quando o proprietário sai de escopo, o valor é descartado**

Quando a variável que possui um valor sai do escopo (termina sua execução), Rust automaticamente limpa a memória desse valor.

**Exemplo:**
```rust
fn main() {
    {
        let s = String::from("Desaparecendo");
        // `s` é válido dentro deste bloco.
    } // `s` sai de escopo aqui e a memória é liberada.

    // println!("{}", s); // Erro! `s` não existe mais.
}
```

## **Regras de Empréstimo (Borrowing)**

1. **Você pode ter múltiplas referências imutáveis ou uma única referência mutável, mas não ambas simultaneamente.**
2. **Referências devem sempre ser válidas.**

### **1. Múltiplas referências imutáveis ou uma única referência mutável**

- **Referências Imutáveis (`&T`):** Permitem ler o valor sem modificá-lo. Você pode ter várias referências imutáveis ao mesmo tempo.
  
  **Exemplo:**
  ```rust
  fn main() {
      let s = String::from("Olá");

      let r1 = &s;
      let r2 = &s;
      let r3 = &s;

      println!("{}, {}, e {}", r1, r2, r3); // Funciona perfeitamente.
  }
  ```

- **Referência Mutável (`&mut T`):** Permite modificar o valor. Apenas uma referência mutável pode existir por vez, e não pode coexistir com referências imutáveis.

  **Exemplo:**
  ```rust
  fn main() {
      let mut s = String::from("Olá");

      let r1 = &mut s; // Única referência mutável.

      r1.push_str(", Mundo!");

      println!("{}", r1); // Funciona.
  }
  ```

- **Tentando misturar referências mutáveis e imutáveis:**
  
  **Exemplo com Erro:**
  ```rust
  fn main() {
      let mut s = String::from("Olá");

      let r1 = &s; // Referência imutável.
      let r2 = &mut s; // Erro! Não pode ter referência mutável enquanto referências imutáveis existem.

      println!("{}, {}", r1, r2);
  }
  ```
  **Erro de Compilação:**
  ```
  error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
   --> src/main.rs:6:14
    |
  4 |     let r1 = &s;
    |              -- immutable borrow occurs here
  5 |     let r2 = &mut s;
    |              ^^^^^ mutable borrow occurs here
  6 |     println!("{}, {}", r1, r2);
    |                        ^^ immutable borrow later used here
  ```

### **2. Referências devem sempre ser válidas**

As referências devem apontar para valores que ainda estão válidos. Rust garante isso durante a compilação para evitar referências pendentes (dangling references).

**Exemplo com Erro:**
```rust
fn main() {
    let r;
    {
        let s = String::from("Desaparecendo");
        r = &s;
    } // `s` sai de escopo aqui.
    // println!("{}", r); // Erro! `s` não existe mais.
}
```
**Erro de Compilação:**
```
error[E0597]: `s` does not live long enough
 --> src/main.rs:6:13
  |
5 |         let s = String::from("Desaparecendo");
  |             - `s` declared here
6 |         r = &s;
  |             ^^ borrowed value does not live long enough
7 |     }
  |     - `s` dropped here while still borrowed
8 |     println!("{}", r);
  |                    - borrow later used here
```

## **Resumo das Principais Regras**

1. **Propriedade (Ownership):**
   - Cada valor possui um único proprietário.
   - Transferir a propriedade (movimento) invalida a variável original.
   - Quando o proprietário sai de escopo, o valor é descartado.

2. **Empréstimo (Borrowing):**
   - Você pode ter múltiplas referências imutáveis ou uma única referência mutável.
   - Não é permitido misturar referências mutáveis e imutáveis ao mesmo tempo.
   - Referências devem sempre apontar para valores válidos.

## **Por que Essas Regras Existem?**

Essas regras de propriedade e empréstimo garantem que Rust gerencie a memória de forma segura e eficiente, evitando condições de corrida, acessos inválidos e vazamentos de memória, tudo sem a necessidade de um coletor de lixo (garbage collector).

Entender e seguir essas regras é fundamental para aproveitar ao máximo a segurança e a performance que Rust oferece.

## Copy e Clone

Claro! Vamos explorar os **traits `Copy` e `Clone`** em Rust utilizando uma **struct simples**. Explicarei de maneira clara e com exemplos para facilitar o entendimento.

---

## O que são Traits `Copy` e `Clone`?**

### **Trait `Copy`**

- **Definição:** 
  - O trait `Copy` permite que tipos implementem uma **cópia por bit** (bitwise copy). Isso significa que, ao atribuir ou passar esses valores, uma cópia completa é feita automaticamente.
  
- **Características:**
  - Tipos que implementam `Copy` não requerem uma chamada explícita para copiar; a cópia acontece automaticamente.
  - Para um tipo implementar `Copy`, **todos os seus campos também devem implementar `Copy`**.
  - É usado para tipos **simples e de tamanho fixo**, como inteiros (`i32`), `bool`, `char`, etc.

### **Trait `Clone`**

- **Definição:**
  - O trait `Clone` permite criar **cópias explícitas** de valores. Ele define o método `.clone()` que você pode chamar para duplicar um valor.

- **Características:**
  - Pode ser implementado para tipos que precisam de uma cópia profunda ou personalizada.
  - **Não é automático** como `Copy`; você deve chamar `.clone()` quando desejar duplicar o valor.
  - Útil para tipos mais complexos, como `String` e `Vec<T>`, que não implementam `Copy`.

---

## Diferença Entre `Copy` e `Clone`**

- **`Copy`:**
  - Cópia automática e implícita.
  - Requer que todos os componentes também sejam `Copy`.
  - Usado para tipos simples e de baixo custo para copiar.

- **`Clone`:**
  - Cópia explícita e controlada pelo programador.
  - Pode ser implementado para tipos complexos.
  - Permite implementações personalizadas de cópia.

## Exemplo Prático com uma Struct Simples

Vamos criar uma struct chamada `Ponto` que representa um ponto 2D com coordenadas `x` e `y`.

### Implementando `Copy` e `Clone`

```rust
#[derive(Debug, Copy, Clone)]
struct Ponto {
    x: i32,
    y: i32,
}

fn main() {
    let ponto1 = Ponto { x: 10, y: 20 };
    
    // Usando `Copy`
    let ponto2 = ponto1; // `ponto1` ainda é válido porque `Ponto` implementa `Copy`
    
    println!("ponto1: {:?}", ponto1);
    println!("ponto2: {:?}", ponto2);
    
    // Usando `Clone`
    let ponto3 = ponto1.clone(); // Cria uma cópia explícita de `ponto1`
    
    println!("ponto3: {:?}", ponto3);
}
```

### Explicação do Código

1. **Derivando `Copy` e `Clone`:**
   ```rust
   #[derive(Debug, Copy, Clone)]
   struct Ponto {
       x: i32,
       y: i32,
   }
   ```
   - Usamos `#[derive(Debug, Copy, Clone)]` para automaticamente implementar os traits `Copy` e `Clone` para a struct `Ponto`.
   - Como `i32` implementa `Copy`, a struct `Ponto` também pode implementar `Copy`.

2. **Usando `Copy`:**
   ```rust
   let ponto2 = ponto1; // Cópia automática
   ```
   - Atribuição de `ponto1` para `ponto2` cria uma cópia completa de `ponto1`.
   - Ambas as variáveis (`ponto1` e `ponto2`) são válidas e independentes.

3. **Usando `Clone`:**
   ```rust
   let ponto3 = ponto1.clone(); // Cópia explícita
   ```
   - Chama o método `.clone()` para criar uma cópia de `ponto1`.
   - Útil quando você quer deixar claro que está criando uma nova instância.

4. **Imprimindo os Pontos:**
   ```rust
   println!("ponto1: {:?}", ponto1);
   println!("ponto2: {:?}", ponto2);
   println!("ponto3: {:?}", ponto3);
   ```
   - Usa o trait `Debug` para imprimir os valores das structs.

### Saída do Programa:
```
ponto1: Ponto { x: 10, y: 20 }
ponto2: Ponto { x: 10, y: 20 }
ponto3: Ponto { x: 10, y: 20 }
```

## Quando Usar `Copy` ou `Clone`?

### Use `Copy` Quando:

- O tipo é pequeno e simples (como inteiros, floats, chars, etc.).
- A cópia é barata e não envolve alocação de memória dinâmica.
- Você deseja que a cópia aconteça automaticamente sem necessidade de chamar `.clone()`.

### Use `Clone` Quando:

- O tipo possui dados complexos ou alocação dinâmica (como `String`, `Vec<T>`, etc.).
- Você precisa de uma cópia profunda ou personalizada.
- Deseja ter controle explícito sobre quando a cópia ocorre.

### Exemplo com Tipo que Não Implementa `Copy`

Vamos ver o que acontece quando tentamos usar `Copy` com um tipo que não o implementa, como `String`.

```rust
#[derive(Debug, Clone)]
struct Pessoa {
    nome: String,
    idade: u32,
}

fn main() {
    let pessoa1 = Pessoa {
        nome: String::from("Alice"),
        idade: 30,
    };
    
    // let pessoa2 = pessoa1; // Movimento: `pessoa1` não é mais válido
    let pessoa2 = pessoa1.clone(); // Cópia explícita
    
    println!("pessoa1: {:?}", pessoa1);
    println!("pessoa2: {:?}", pessoa2);
}
```

### Explicação:

- **Movimento vs. Clone:**
  - Se tentarmos atribuir `pessoa1` para `pessoa2` sem implementar `Copy`, ocorrerá um **movimento**, e `pessoa1` não poderá mais ser usado.
  - Com `Clone`, podemos criar uma cópia explícita, mantendo `pessoa1` válida.

### **Saída do Programa:**
```
pessoa1: Pessoa { nome: "Alice", idade: 30 }
pessoa2: Pessoa { nome: "Alice", idade: 30 }
```

## Resumo

- **`Copy`:**
  - Cópia automática e implícita.
  - Usado para tipos simples que implementam `Copy`.
  - Exemplos: `i32`, `f64`, `bool`, `char`.

- **`Clone`:**
  - Cópia explícita controlada pelo programador.
  - Usado para tipos mais complexos ou que requerem cópias personalizadas.
  - Exemplos: `String`, `Vec<T>`, structs complexas.

- **Implementação em Structs:**
  - Para derivar `Copy`, todos os campos da struct devem implementar `Copy`.
  - `Clone` pode ser derivado independentemente, permitindo cópias profundas ou personalizadas.

Entender a diferença entre `Copy` e `Clone` ajuda a gerenciar a propriedade e a eficiência do seu código em Rust, garantindo que você esteja fazendo cópias de forma apropriada conforme a complexidade dos tipos que está utilizando.

## **1. Propriedade e Empréstimo com Structs**

### **a. Entendendo Structs em Rust**

Uma **struct** (estrutura) em Rust é uma forma de agrupar diferentes valores sob um mesmo nome. Por exemplo:

```rust
struct Pessoa {
    nome: String,
    idade: u32,
}
```

### **b. Propriedade em Structs**

Quando você cria uma instância de uma struct, a variável que a possui é o **proprietário** dos dados dentro dela. Vamos ver um exemplo:

```rust
fn main() {
    let pessoa1 = Pessoa {
        nome: String::from("Alice"),
        idade: 30,
    };

    // Movendo a propriedade de pessoa1 para pessoa2
    let pessoa2 = pessoa1;

    // println!("{}", pessoa1.nome); // Erro! pessoa1 não é mais válido
    println!("Nome: {}, Idade: {}", pessoa2.nome, pessoa2.idade);
}
```

**Explicação:**

- **Movimento de Propriedade:**
  - Ao atribuir `pessoa1` para `pessoa2`, a propriedade dos dados é **movida** para `pessoa2`.
  - Após a movimentação, **`pessoa1` não pode mais ser usado**, pois `String` **não implementa** o trait `Copy`.

### **c. Empréstimo Imutável em Structs**

Você pode **emprestar** uma referência imutável a uma struct sem transferir a propriedade:

```rust
fn main() {
    let pessoa = Pessoa {
        nome: String::from("Bob"),
        idade: 25,
    };

    imprimir_pessoa(&pessoa);

    // pessoa ainda é válido aqui
    println!("Nome após empréstimo: {}", pessoa.nome);
}

fn imprimir_pessoa(p: &Pessoa) {
    println!("Nome: {}, Idade: {}", p.nome, p.idade);
}
```

**Explicação:**

- **Referência Imutável (`&Pessoa`):**
  - A função `imprimir_pessoa` recebe uma referência imutável à `Pessoa`.
  - Você pode ter múltiplas referências imutáveis simultaneamente.
  - **A propriedade não é transferida**, então `pessoa` permanece válido após o empréstimo.

### **d. Empréstimo Mutável em Structs**

Para modificar os dados de uma struct, você pode emprestar uma referência mutável:

```rust
fn main() {
    let mut pessoa = Pessoa {
        nome: String::from("Carol"),
        idade: 28,
    };

    atualizar_idade(&mut pessoa);

    println!("Nome: {}, Idade: {}", pessoa.nome, pessoa.idade);
}

fn atualizar_idade(p: &mut Pessoa) {
    p.idade += 1;
}
```

**Explicação:**

- **Referência Mutável (`&mut Pessoa`):**
  - A função `atualizar_idade` recebe uma referência mutável à `Pessoa`.
  - Apenas **uma referência mutável** pode existir por vez.
  - Permite modificar os dados dentro da struct.

### **e. Regras Principais de Empréstimo em Structs**

1. **Múltiplas Referências Imutáveis:**
   - Você pode ter várias referências imutáveis (`&Pessoa`) ao mesmo tempo.
   - Nenhuma dessas referências pode modificar os dados.

2. **Uma Única Referência Mutável:**
   - Apenas uma referência mutável (`&mut Pessoa`) pode existir em um determinado momento.
   - Não pode coexistir com referências imutáveis enquanto a referência mutável está ativa.

3. **Referências Válidas:**
   - As referências devem apontar para dados válidos. Rust verifica isso em tempo de compilação para evitar erros de acesso a memória inválida.

## **2. Propriedade e Empréstimo com Vecs**

### **a. Entendendo Vecs em Rust**

Um **Vec** (`Vec<T>`) é uma coleção dinâmica que pode armazenar múltiplos elementos do mesmo tipo. Por exemplo:

```rust
fn main() {
    let numeros = vec![1, 2, 3, 4, 5];
}
```

### **b. Propriedade em Vecs**

Similar às structs, quando você atribui um `Vec` a outra variável, a propriedade é movida:

```rust
fn main() {
    let vec1 = vec![10, 20, 30];

    let vec2 = vec1; // Movimento: vec1 deixa de ser válido

    // println!("{:?}", vec1); // Erro! vec1 não é mais válido
    println!("{:?}", vec2);
}
```

**Explicação:**

- **Movimento de Propriedade:**
  - `vec1` é movido para `vec2`.
  - Após a movimentação, `vec1` não pode mais ser usado.

### **c. Empréstimo Imutável em Vecs**

Você pode emprestar uma referência imutável a um `Vec` para ler seus dados sem transferir a propriedade:

```rust
fn main() {
    let numeros = vec![100, 200, 300];

    imprimir_numeros(&numeros);

    // numeros ainda é válido aqui
    println!("Primeiro número: {}", numeros[0]);
}

fn imprimir_numeros(v: &Vec<i32>) {
    for num in v {
        println!("{}", num);
    }
}
```

**Explicação:**

- **Referência Imutável (`&Vec<i32>`):**
  - A função `imprimir_numeros` recebe uma referência imutável ao `Vec`.
  - Permite ler os dados sem modificar.

### **d. Empréstimo Mutável em Vecs**

Para modificar um `Vec`, você pode emprestar uma referência mutável:

```rust
fn main() {
    let mut numeros = vec![1, 2, 3];

    adicionar_numero(&mut numeros, 4);

    println!("{:?}", numeros);
}

fn adicionar_numero(v: &mut Vec<i32>, num: i32) {
    v.push(num);
}
```

**Explicação:**

- **Referência Mutável (`&mut Vec<i32>`):**
  - A função `adicionar_numero` recebe uma referência mutável ao `Vec`.
  - Permite modificar o `Vec`, adicionando um novo elemento.

### **e. Regras Principais de Empréstimo em Vecs**

As regras são as mesmas aplicadas às structs:

1. **Múltiplas Referências Imutáveis:**
   - Você pode ter várias referências imutáveis (`&Vec<T>`) ao mesmo tempo.

2. **Uma Única Referência Mutável:**
   - Apenas uma referência mutável (`&mut Vec<T>`) pode existir em um determinado momento.
   - Não pode coexistir com referências imutáveis enquanto a referência mutável está ativa.

3. **Referências Válidas:**
   - As referências devem apontar para dados válidos e não podem viver além dos dados que referenciam.

---

## **3. Exemplos Combinados com Structs e Vecs**

Vamos ver como structs que contêm `Vecs` interagem com as regras de propriedade e empréstimo:

```rust
struct Biblioteca {
    livros: Vec<String>,
}

fn main() {
    let mut biblioteca = Biblioteca {
        livros: vec![String::from("Rust 101"), String::from("The Book")],
    };

    adicionar_livro(&mut biblioteca, String::from("Programming Rust"));

    imprimir_livros(&biblioteca);

    // Movendo a biblioteca para outra variável
    let biblioteca2 = biblioteca;

    // println!("{:?}", biblioteca.livros); // Erro! biblioteca não é mais válido
    println!("{:?}", biblioteca2.livros);
}

fn adicionar_livro(bib: &mut Biblioteca, livro: String) {
    bib.livros.push(livro);
}

fn imprimir_livros(bib: &Biblioteca) {
    for livro in &bib.livros {
        println!("{}", livro);
    }
}
```

**Explicação:**

1. **Struct `Biblioteca`:**
   - Possui um campo `livros` que é um `Vec<String>`.

2. **Empréstimo Mutável para Adicionar Livro:**
   - A função `adicionar_livro` recebe uma referência mutável à `Biblioteca` para adicionar um novo livro.
   - Permite modificar o `Vec` dentro da struct.

3. **Empréstimo Imutável para Imprimir Livros:**
   - A função `imprimir_livros` recebe uma referência imutável à `Biblioteca` para ler e imprimir os livros.
   - Permite múltiplas leituras sem modificar.

4. **Movimento de Propriedade:**
   - Ao atribuir `biblioteca` para `biblioteca2`, a propriedade é movida.
   - `biblioteca` não pode mais ser usado após a movimentação.

## **4. Dicas para Lidar com Propriedade e Empréstimo**

1. **Use Referências Sempre que Possível:**
   - Para evitar movimentos desnecessários e manter a variável original válida, utilize referências (`&` ou `&mut`) em vez de transferir a propriedade.

2. **Mantenha as Structs e Vecs Mutáveis Quando Necessário:**
   - Se precisar modificar os dados dentro de uma struct ou Vec, declare-os como `mut` e use referências mutáveis.

3. **Evite Múltiplas Referências Mutáveis:**
   - Lembre-se de que apenas uma referência mutável pode existir por vez para evitar conflitos e garantir a segurança de dados.

4. **Clone Quando Necessário:**
   - Se precisar de uma cópia dos dados sem mover a propriedade, utilize o método `.clone()`. Porém, tenha em mente que isso pode ter um custo de performance dependendo do tamanho dos dados.

   ```rust
   let biblioteca_clone = biblioteca2.clone();
   ```

5. **Entenda o Escopo:**
   - As regras de propriedade e empréstimo são aplicadas no **tempo de compilação**, garantindo que todas as referências sejam válidas e que não haja acessos inválidos à memória.

## Então...

- **Propriedade (Ownership):**
  - Cada valor tem um único proprietário.
  - Movimentar a propriedade transfere-a para outra variável, invalidando a original.
  
- **Empréstimo (Borrowing):**
  - **Imutável (`&T`):** Permite múltiplas leituras sem modificar.
  - **Mutável (`&mut T`):** Permite uma única modificação, sem coexistência com referências imutáveis.

- **Structs e Vecs:**
  - Seguem as mesmas regras de propriedade e empréstimo.
  - Utilize referências para evitar movimentos desnecessários.
  - Utilize mutabilidade (`mut`) quando precisar modificar os dados.

Entender e aplicar corretamente as regras de propriedade e empréstimo em Rust garante que seu código seja seguro, eficiente e livre de erros comuns relacionados a gestão de memória.

## **Por Que Precisamos de Lifetimes em Structs com Referências?**

**Lifetimes** garantem que as referências dentro de uma struct sejam válidas enquanto a struct estiver em uso. Eles evitam que a struct contenha referências para dados que já foram descartados, prevenindo erros de memória.

### **Principais Motivos:**

1. **Segurança de Memória:** Asseguram que referências não se tornem inválidas (dangling references).
2. **Relacionamento de Escopo:** Definem quanto tempo as referências devem viver em relação à struct.
3. **Verificação pelo Compilador:** O compilador Rust usa lifetimes para garantir que todas as referências são válidas.

## **Exemplos Simples**

### **1. Struct com Referências e Lifetimes**

```rust
struct Pessoa<'a> {
    nome: &'a str,
}

fn main() {
    let nome = String::from("Alice");
    let pessoa = Pessoa { nome: &nome };
    println!("Nome: {}", pessoa.nome);
}
```

**Explicação:**

- `'a` é um lifetime que indica que a referência `nome` na struct `Pessoa` deve viver pelo menos tanto quanto `'a`.
- O compilador garante que `nome` não será descartado enquanto `pessoa` estiver em uso.

### **2. Struct Sem Lifetimes (Causa Erro)**

```rust
struct Pessoa {
    nome: &str, // Erro: falta especificar o lifetime
}

fn main() {
    let nome = String::from("Bob");
    let pessoa = Pessoa { nome: &nome };
    println!("Nome: {}", pessoa.nome);
}
```

**Erro do Compilador:**
```
error[E0106]: missing lifetime specifier
 --> src/main.rs:1:14
  |
1 | struct Pessoa {
  |              ^ expected named lifetime parameter
```

**Explicação:**

- Rust requer a especificação de lifetimes para referências em structs para saber como gerenciar a validade das referências.

### **3. Alternativa: Possuir os Dados (Sem Referências)**

```rust
struct Pessoa {
    nome: String, // Possui os dados, sem necessidade de lifetimes
}

fn main() {
    let pessoa = Pessoa { nome: String::from("Carol") };
    println!("Nome: {}", pessoa.nome);
}
```

**Vantagens:**

- **Sem Lifetimes Necessários:** Como a struct `Pessoa` possui o `String`, não há referências e, portanto, não são necessários lifetimes.
- **Maior Flexibilidade:** A struct pode existir independentemente das variáveis externas.

## **Resumo Rápido**

- **Lifetimes** são necessários em structs que contêm referências para garantir que as referências sejam válidas enquanto a struct estiver em uso.
- **Anote lifetimes** usando a sintaxe `<'a>` quando definir structs com referências.
- **Alternativa:** Em vez de usar referências, faça a struct **possuir os dados** (por exemplo, use `String` em vez de `&str`) para evitar a necessidade de lifetimes.

Sempre que possível, prefira que structs possuam seus próprios dados para simplificar o gerenciamento de memória e evitar a complexidade adicional dos lifetimes. Use referências com lifetimes quando realmente precisar compartilhar dados sem duplicá-los.

## **1. `Rc<>` (Reference Counted)**

### **O Que É?**
- **`Rc<T>`** é um *smart pointer* que permite **múltiplas referências** a um único dado.
- Utiliza **contagem de referências** para rastrear quantas `Rc` apontam para o mesmo valor.

### **Quando Usar?**
- Quando você precisa **compartilhar dados** entre múltiplas partes do seu programa **sem modificar** esses dados.
- **Apenas para uso em **single-threaded** (não seguro para múltiplas threads).

### **Exemplo Simples:**
```rust
use std::rc::Rc;

fn main() {
    let dados = Rc::new(5);
    let referencia1 = Rc::clone(&dados);
    let referencia2 = Rc::clone(&dados);

    println!("Dados: {}", dados);
    println!("Referência 1: {}", referencia1);
    println!("Referência 2: {}", referencia2);
}
```
**Saída:**
```
Dados: 5
Referência 1: 5
Referência 2: 5
```

## **2. `RefCell<>`**

### **O Que É?**
- **`RefCell<T>`** permite **mutabilidade interior**, ou seja, permite modificar dados mesmo quando você tem uma referência imutável.
- Realiza **verificações de empréstimo em tempo de execução**, ao contrário das verificações em tempo de compilação.

### **Quando Usar?**
- Quando você precisa **alterar dados** que são referenciados de maneira imutável.
- **Dentro de um único thread**, pois `RefCell` não é seguro para múltiplas threads.

### **Exemplo Simples:**
```rust
use std::cell::RefCell;

fn main() {
    let dados = RefCell::new(5);
    
    // Empréstimo mutável
    *dados.borrow_mut() += 1;
    
    println!("Dados: {}", dados.borrow());
}
```
**Saída:**
```
Dados: 6
```

## **3. `Rc<RefCell<>>` (Combinação de `Rc` e `RefCell`)**

### **O Que É?**
- Combina **`Rc`** para **múltiplas referências** com **`RefCell`** para **mutabilidade interior**.
- Permite que **múltiplas partes** do seu programa **possam **compartilhar e **modificar** o mesmo dado.

### **Quando Usar?**
- Quando você precisa **compartilhar e modificar** dados entre várias partes do programa.
- **Com cuidado**, pois combina **contagem de referências** com **mutabilidade dinâmica**, o que pode introduzir complexidade.

### **Exemplo Simples:**
```rust
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let dados = Rc::new(RefCell::new(5));
    
    let referencia1 = Rc::clone(&dados);
    let referencia2 = Rc::clone(&dados);
    
    // Modificando através de uma referência
    *referencia1.borrow_mut() += 10;
    
    println!("Dados via referencia1: {}", referencia1.borrow());
    println!("Dados via referencia2: {}", referencia2.borrow());
    println!("Dados originais: {}", dados.borrow());
}
```
**Saída:**
```
Dados via referencia1: 15
Dados via referencia2: 15
Dados originais: 15
```

## **Resumo Rápido**

- **`Rc<T>`:**
  - Permite **múltiplas referências** a dados imutáveis.
  - **Contagem de referências** gerencia a memória.
  - **Single-threaded**.

- **`RefCell<T>`:**
  - Permite **mutabilidade interior**.
  - **Verificações de empréstimo** em **tempo de execução**.
  - **Single-threaded**.

- **`Rc<RefCell<T>>`:**
  - Combina **múltiplas referências** com **mutabilidade interior**.
  - Permite **compartilhar e modificar** dados entre várias partes.
  - **Single-threaded**.

## **Dicas Importantes**

1. **Evite Ciclos de Referência:**
   - Usar `Rc` pode levar a **vazamentos de memória** se houver ciclos de referência. Utilize `Weak` para quebrar ciclos quando necessário.

2. **Cuidado com Erros em Tempo de Execução:**
   - `RefCell` pode **panicar** se as regras de empréstimo forem violadas (como tentar emprestar mutavelmente enquanto há empréstimos imutáveis ativos).

3. **Preferir Segurança de Compilação:**
   - Sempre que possível, prefira soluções que utilizem as verificações de empréstimo em tempo de compilação (`Rc` com apenas referências imutáveis ou `&mut` com referências mutáveis) antes de recorrer a `RefCell`.
