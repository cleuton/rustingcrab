**Correção:**

1. **Match como "switch":**  
   Crie uma função que recebe um inteiro entre 1 e 3 e, usando `match`, imprima no console uma mensagem diferente para cada valor (por exemplo, 1 -> "Um", 2 -> "Dois", 3 -> "Três"). Caso o valor não seja 1, 2 ou 3, imprima "Valor fora do intervalo".

2. **Match com guard (condição de guarda):**  
   Crie uma função que recebe um número inteiro qualquer e, usando `match` com guard, verifique o seguinte:  
   - Se o número for negativo, imprima "Número negativo".  
   - Se for entre 0 e 10 (incluindo ambos), imprima "Entre 0 e 10".  
   - Se for maior que 10, imprima "Maior que 10".

3. **Match com desestruturação de tuplas:**  
   Crie uma função que recebe uma tupla `(i32, i32)` e, usando `match`, faça:  
   - Se ambos os valores forem iguais, imprima "Os valores são iguais".  
   - Se o primeiro for maior que o segundo, imprima "Primeiro maior".  
   - Caso contrário, imprima "Segundo maior".

---

**Soluções e Explicações:**

**Exercício 1: Match como "switch"**

```rust
fn imprimir_numero(n: i32) {
    match n {
        1 => println!("Um"),
        2 => println!("Dois"),
        3 => println!("Três"),
        _ => println!("Valor fora do intervalo"),
    }
}

fn main() {
    imprimir_numero(1); // Imprime "Um"
    imprimir_numero(4); // Imprime "Valor fora do intervalo"
}
```

**Explicação:**  
O `match` aqui funciona como um "switch" de outras linguagens. Cada padrão (1, 2, 3) mapeia para um braço diferente. O `_` é um "coringa" que corresponde a qualquer outro valor que não bateu nos anteriores.

---

**Exercício 2: Match com guard**

```rust
fn verificar_numero(n: i32) {
    match n {
        x if x < 0 => println!("Número negativo"),
        x if x >= 0 && x <= 10 => println!("Entre 0 e 10"),
        _ => println!("Maior que 10"),
    }
}

fn main() {
    verificar_numero(-5); // Imprime "Número negativo"
    verificar_numero(7);  // Imprime "Entre 0 e 10"
    verificar_numero(20); // Imprime "Maior que 10"
}
```

**Explicação:**  
Aqui usamos `if` dentro do `match`, o que chamamos de guard. Isso permite adicionar condições extras além de apenas checar padrões. O valor `n` é testado nas guardas para decidir qual braço será executado.

---

**Exercício 3: Match com desestruturação de tuplas**

```rust
fn comparar_tupla(valores: (i32, i32)) {
    match valores {
        (x, y) if x == y => println!("Os valores são iguais"),
        (x, y) if x > y  => println!("Primeiro maior"),
        _                => println!("Segundo maior"),
    }
}

fn main() {
    comparar_tupla((5, 5));   // Imprime "Os valores são iguais"
    comparar_tupla((10, 2));  // Imprime "Primeiro maior"
    comparar_tupla((2, 10));  // Imprime "Segundo maior"
}
```

**Explicação:**  
Aqui estamos desestruturando a tupla `(i32, i32)` diretamente no `match`. Ao escrever `(x, y)`, extraímos os valores da tupla em duas variáveis separadas. A partir daí, aplicamos condições usando guard (`if x == y`, `if x > y`) para decidir qual mensagem imprimir. Desse modo, conseguimos acessar elementos internos da estrutura e tomar decisões de acordo com o conteúdo dela.