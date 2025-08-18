<img src="../../rusting-crab-logo.png" height=300>

---

[**Cleuton Sampaio**](https://linkedin.com/in/cleutonsampaio)

[**Veja no GitHub**](https://https://github.com/cleuton/rustingcrab/tree/main/code_samples/coroutines)

# Coroutines 

Coroutines são “funções que podem pausar e continuar depois”. Em vez de rodar até o fim de uma vez, elas podem fazer um `yield` (entregar um valor parcial), parar ali, e mais tarde retomar exatamente do ponto em que pararam.

Para que servem?

* Produzir dados **aos poucos** sem criar tudo de uma vez (ex.: gerar números primos em sequência).
* Implementar **fluxos/iteradores** complexos de forma direta, sem estados manuais espalhados.
* Fazer **trabalho cooperativo**: uma parte avança um pouco, cede o controle, e depois continua.
* Em Rust, são o **mecanismo de base** semelhante ao que `async/await` usa por baixo (máquina de estados gerada pelo compilador).

No Rust atual, coroutines ainda são **experimentais** (nightly): você marca uma *closure* especial que pode dar `yield`, e quem consome chama “retomar” para continuar a execução até o próximo `yield` ou até terminar.

Apesar do nome, as **coroutines** do Rust se parecem **mais com os generators do Python** do que com **goroutines** do **Go**.

## Comparando com Generators (Python) e Goroutines (GO)

* **Rust coroutines (nightly)**: funções “retomáveis” e **stackless**; você chama `resume(...)`, elas podem liberar (`yield`) valores várias vezes e depois **completam**. Não rodam em paralelo por si, avançando quando você manda. São um *building block* de baixo nível (sem runtime).
* **Python generators**: mesma ideia cooperativa: `next()` ou `.send()` avança, `yield` devolve valores, depois finaliza. Também **não** há paralelismo intrínseco.
* **Go goroutines**: unidades de execução **concorrentes** leves, com **agendador** do runtime de Go. Você cria `go f()`, e o runtime executa em paralelo (multiplexa em threads), com **preempção**.

## Diferenças-chave (em poucas linhas)

* **Concorrência**:

  * Rust coroutine: não; só avança quando você `resume`.
  * Python generator: não; só avança em `next/send`.
  * Go goroutine: **sim**; roda concorrente sob um scheduler.
* **Agendamento**:

  * Rust/Python: **cooperativo** (quem chama decide quando avançar).
  * Go: **preemptivo/cooperativo** via runtime; o scheduler decide quando rodar.
* **Memória/forma**:

  * Rust/Python: **stackless** (compilador/VM geram uma máquina de estados).
  * Go: **stackful** (“pilha” leve que cresce/encolhe por goroutine).
* **Runtime**:

  * Rust coroutine: **sem runtime** embutido; zero-cost abstractions, controle explícito.
  * Python generator: executado pela VM do Python.
  * Go goroutine: requer o **runtime** de Go (scheduler, GC, etc.).
* **Uso típico**:

  * Rust/Python: iteradores complexos, *pipelines*, parsing, produzir dados aos poucos.
  * Go: I/O concorrente, servidores, *workers*, pipelines concorrentes com *channels*.
* **Comunicação**:

  * Rust coroutine: “comunica” via valores de `yield`/retorno (você compõe manualmente).
  * Python generator: idem (e tem `.throw()`, `.close()`).
  * Go goroutine: **channels** e *select* nativos.
* **Paralelismo real**:

  * Rust/Python generators/coroutines: só se você integrá-los a *threads*/executores.
  * Go: já vem pronto para rodar concorrente/ paralelo (conforme threads disponíveis).

### Regra de bolso

* Precisa **concorrência fácil** e escalável? Pense em **goroutines** (Go).
* Quer **produzir sequência de valores** de forma legível, sem montar estado manual? **Generators (Python)** ou **coroutines (Rust)**.
* Em Rust, coroutines hoje são **experimentais (nightly)** e servem como tijolo de baixo nível; para I/O concorrente no Rust estável, use **`async/await` + executor**.

## Tem que usar o toolchain nightly

Como as coroutines ainda são **instáveis**: exigem `#![feature(coroutines, coroutine_trait, yield_expr)]` e a anotação `#[coroutine]`, o compilador **só aceita na versão nightly do Rust**. No Rust estável, *feature gates* não são permitidos, então o código não compila.

Para instalar e usar o nightly sem “quebrar” seu `stable`:

```bash
rustup toolchain install nightly
rustup default stable            # mantém o stable como padrão
cargo +nightly run               # usa nightly só neste comando
```

Se preferir **não** depender de `+nightly` na linha de comando, fixe o toolchain no repositório com `rust-toolchain.toml`:

```toml
# rust-toolchain.toml
[toolchain]
channel = "nightly"              # ou um snapshot específico, ex.: "nightly-2025-08-17"
components = ["rustc", "cargo", "clippy", "rustfmt"]
profile = "minimal"
```

Com esse arquivo, dentro do projeto `cargo build` já usa o nightly automaticamente, enquanto todos os seus outros projetos continuam no `stable`.

## Exemplo


Segue um exemplo claro usando **coroutines** para gerar primos com o **Crivo de Eratóstenes** até um limite dado. é um projeto mínimo completo (3 arquivos). funciona no **nightly** e não mexe no seu `stable`.

Arquivo: `rust-toolchain.toml`

```toml
[toolchain]
channel = "nightly"
components = ["rustc", "cargo", "clippy", "rustfmt"]
profile = "minimal"
```

Arquivo: `Cargo.toml`

```toml
[package]
name = "sieve-coroutines"
version = "0.1.0"
edition = "2024"

[dependencies]
```

Arquivo: `src/main.rs`

```rust
#![feature(coroutines, coroutine_trait, yield_expr, stmt_expr_attributes)]

use std::ops::{Coroutine, CoroutineState};
use std::pin::Pin;

fn main() {
    let limit = 100;

    // coroutine que gera primos até `limit`
    let mut primes = #[coroutine] move || {
        if limit < 2 {
            return ();
        }

        // crivo clássico
        let mut is_prime = vec![true; limit + 1];
        is_prime[0] = false;
        is_prime[1] = false;

        let mut i = 2usize;
        while i * i <= limit {
            if is_prime[i] {
                let mut m = i * i;
                while m <= limit {
                    is_prime[m] = false;
                    m += i;
                }
            }
            i += 1;
        }

        // produz os primos com `yield`
        for p in 2..=limit {
            if is_prime[p] {
                yield p; // suspende e entrega `p`
            }
        }

        // retorno final (sem valor extra)
        ()
    };

    // consome a coroutine até completar
    print!("primes up to {limit}:");
    loop {
        match Pin::new(&mut primes).resume(()) {
            CoroutineState::Yielded(p) => print!(" {p}"),
            CoroutineState::Complete(()) => break,
        }
    }
    println!();
}
```

como usar: só rodar `cargo run` dentro desse projeto. o `rust-toolchain.toml` faz o cargo usar **nightly só aqui**, sem tocar no seu padrão `stable`.
