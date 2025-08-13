<img src="./brincando.png" height=400>

---

<img src="../../rusting-crab-logo.png" height=300>

---

[**Cleuton Sampaio**](https://linkedin.com/in/cleutonsampaio)

[**Veja no GitHub**](https://https://github.com/cleuton/rustingcrab/tree/main/code_samples/rust_ffi)

# Guia de Integração: Rust com Java, Python e Node.js via FFI

Este projeto demonstra como integrar código escrito em Rust com aplicações desenvolvidas em Java, Python e Node.js utilizando Foreign Function Interface (FFI). O objetivo é mostrar que é possível introduzir Rust de forma incremental em sistemas existentes, aproveitando seu desempenho e segurança sem substituir toda a base de código.

Cada exemplo ilustra como compilar uma função Rust como uma biblioteca nativa e chamá-la diretamente a partir de outra linguagem. Embora as ferramentas utilizadas sejam diferentes, o princípio subjacente é o mesmo: comunicação entre linguagens por meio de interfaces nativas.

---

## Estrutura do Projeto

O projeto está organizado em três diretórios principais, cada um contendo um exemplo de integração com uma linguagem diferente:

- rust_jni_demo/     – Integração com Java usando JNI (Java Native Interface)
- pyrust_demo/   – Integração com Python usando PyO3
- node_rust_demo/   – Integração com Node.js usando napi-rs

Cada diretório contém os arquivos-fonte necessários e pode ser construído independentemente.

---

## Java com Rust via JNI

### Descrição
Este exemplo mostra como chamar uma função Rust a partir de uma aplicação Java usando a JNI (Java Native Interface). A função Rust calcula o fatorial de um número inteiro.

### Código Java
```java
public class Calculator {
    static {
        System.loadLibrary("rust_jni_demo");
    }

    public native long factorial(int n);

    public static void main(String[] args) {
        Calculator calc = new Calculator();
        System.out.println(calc.factorial(10));
    }
}
```

A linha `System.loadLibrary("rust_jni_demo")` carrega a biblioteca nativa compilada a partir do código Rust. O método `factorial` é declarado como `native`, indicando que sua implementação está em código nativo.

### Código Rust
```rust
use jni::objects::JObject;
use jni::sys::{jint, jlong};
use jni::JNIEnv;

#[no_mangle]
pub extern "system" fn Java_Calculator_factorial(
    _env: JNIEnv,
    _this: JObject,
    n: jint
) -> jlong {
    if n < 0 {
        return 0;
    }
    if n > 20 {
        return 0;
    }
    let mut acc: i64 = 1;
    for i in 1..=n as i64 {
        acc = acc.saturating_mul(i);
    }
    acc as jlong
}
```

A função Rust é exportada com `#[no_mangle]` para preservar o nome e seguir a convenção de chamada C. O nome da função `Java_Calculator_factorial` segue a convenção JNI: `Java_` + nome da classe + nome do método. Ela recebe parâmetros compatíveis com tipos C e retorna um valor nativo.

### Como compilar e executar
1. Gere o arquivo `Calculator.h` com o comando `javac -h . Calculator.java`.
2. Compile o código Rust com `cargo build --release`.
3. Execute com:
   ```bash
   java -cp . -Djava.library.path=target/release Calculator
   ```

---

## Python com Rust via PyO3

### Descrição
Este exemplo demonstra como criar uma extensão Python em Rust usando a biblioteca `PyO3`. A função `fibonacci` é escrita em Rust, mas pode ser chamada como se fosse uma função Python nativa.

### Código Rust
```rust
use pyo3::prelude::*;

#[pyfunction]
fn fibonacci(n: u32) -> PyResult<u64> {
    if n > 93 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Fibonacci(n) is too large for u64 when n > 93"
        ));
    }
    match n {
        0 => Ok(0),
        1 => Ok(1),
        _ => {
            let mut a = 0u64;
            let mut b = 1u64;
            for _ in 2..=n {
                let temp = a.checked_add(b)
                    .ok_or_else(|| pyo3::exceptions::PyOverflowError::new_err("u64 overflow"))?;
                a = b;
                b = temp;
            }
            Ok(b)
        }
    }
}

#[pymodule]
fn pyrust_demo(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fibonacci, m)?)?;
    Ok(())
}
```

A macro `#[pyfunction]` marca a função `fibonacci` para exportação ao Python. O módulo `pyrust_demo` é o ponto de entrada carregado pelo interpretador Python. O uso de `PyResult` permite retornar erros compatíveis com exceções do Python.

### Código Python
```python
from pyrust_demo import fibonacci

print(fibonacci(30))   # Output: 832040
print(fibonacci(94))   # Raises ValueError
```

### Como compilar e executar

Eu sugeriria criar um `venv` antes de mais nada: 

```bash
python -m .venv
source .venv/bin/activate
```

Mas você é quem sabe! 

1. Instale `maturin`: `pip install maturin` ou `pip install -r requirements.txt`.
2. Compile e instale o módulo em modo de desenvolvimento:
   ```bash
   maturin develop
   ```
3. Execute o script Python:
   ```bash
   python test.py
   ```

Não precisa compilar o projeto Rust em separado. 

---

## Node.js com Rust via napi-rs

### Descrição
Este exemplo mostra como criar um addon para Node.js usando `napi-rs`, que permite escrever extensões nativas em Rust com suporte a N-API, garantindo compatibilidade entre versões do Node.js.

### Código Rust
```rust
use napi_derive::napi;

#[napi(js_name = "reverseString")]
pub fn reverse_string(s: String) -> String {
    s.chars().rev().collect()
}
```

A macro `#[napi]` exporta a função para o ambiente Node.js. O atributo `js_name` define como a função será exposta no JavaScript. O tipo `String` é automaticamente convertido entre Rust e JavaScript.

### Código JavaScript
```javascript
const { reverseString } = require('./addon.js');
console.log(reverseString('hello'));
```

O módulo `addon.js` é gerado durante a compilação e atua como um wrapper para o binário nativo.

### Como compilar e executar
1. Instale as dependências:
   ```bash
   npm install
   ```
2. Compile o addon:
   ```bash
   napi build run
   ```
3. Execute:
   ```bash
   node index.js
   ```

---

## Concluindo

Todos os exemplos demonstram o mesmo conceito central: Rust pode ser integrado a sistemas existentes por meio de interfaces nativas. Apesar das diferenças nas ferramentas — JNI, PyO3, napi-rs — o padrão é consistente:

1. Escrever funções em Rust com interface compatível com C
2. Compilar para uma biblioteca compartilhada
3. Carregar e chamar essa biblioteca a partir da linguagem hospedeira

Essa abordagem permite migrações graduais, onde componentes críticos de desempenho são reescritos em Rust, enquanto o restante do sistema permanece funcional.

Este modelo é amplamente utilizado em produção para melhorar eficiência, segurança e confiabilidade sem interromper serviços existentes.
```