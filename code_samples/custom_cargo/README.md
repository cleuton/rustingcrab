<img src="custom_cargo.png" height=400>

---

<img src="../../rusting-crab-logo.png" height=300>

---

[**Cleuton Sampaio**](https://linkedin.com/in/cleutonsampaio)

[**Veja no GitHub**](https://https://github.com/cleuton/rustingcrab/tree/main/code_samples/custom_cargo)

# Comandos cargo e customização

## Usando um script `build.rs`

O que é o `build.rs`? O Cargo o executa automaticamente?

Se você colocar um arquivo chamado **`build.rs`** na raiz do seu projeto (no mesmo nível que o `Cargo.toml`), **o Cargo irá executá-lo automaticamente antes de compilar sua crate**.

> Sim, o Cargo executa qualquer `build.rs`, é uma **convenção**.

### Como o cargo sabe o que deve rodar?

O Cargo segue algumas **convenções de nomenclatura e organização** para entender a estrutura do seu projeto:

| Arquivo            | Propósito |
|-----------------|--------|
| `src/main.rs`   | Binário principal |
| `src/lib.rs`    | Lib principal |
| `tests/*.rs`    | Testes de integração |
| `examples/*.rs` | Binários de exemplos de uso |
| `benches/*.rs`  | Benchmarks (*) |
| **`build.rs`**  | **Script de pré-compilação** |

O Cargo procura por um script `build.rs` na raiz do projeto. Se existir, ele:

1. Compila o `build.rs` em um executável standalone.
2. Executa este executável **antes de compilar seu projeto**.
3. Usa sua saída (variáveis de ambiente, código gerado, flags de linker) durante a compilação.

Ele é chamado de **custom build script**, e lhe dá maior controle sobre a compilação do seu projeto.

### O que você pode fazer em um `build.rs`?

Você pode:

- Gerar código Rust antes da compilação.
- Linkar libs nativas (C/C++/Fortran).
- Detectar a configuração do systema (OS, CPU, etc.).
- Criar variáveis de ambiente para a compilação.
- Controlar quando o Cargo deve re-executar a compilação.
- Mostrar warnings ou erros durante a compilation.

### Example: Gerando código no `OUT_DIR`

Os arquivos gerados devem ser gravados na pasta especificada pela variável de ambiente `OUT_DIR`.

Aqui está um exemplo:

#### `build.rs`

```text
use std::env;
use std::fs;
use std::path::Path;
use chrono::Utc;

fn main() {
    // Obtém a pasta de saída (criada pelo Cargo)
    let dir_saida = env::var("OUT_DIR").expect("Pasta de saída não criada pelo Cargo");
    let caminho_saida = Path::new(&dir_saida).join("saudacao.rs");

    let now = Utc::now();
    let nome = "Fulano";

    // Gera código Rust com a saudação
    let saudacao = format!(
        r#"
        pub fn saudar() -> &'static str {{
            "Ola, {}! Este código foi criado em {}."
        }}
        "#,
        nome,
        now.format("%d/%m/%Y %H:%M:%S")
    );

    // Grava na saída
    fs::write(&caminho_saida, saudacao)
        .expect("Falhou ao gerar o codigo de saudaçao");

    // Avisa o Cargo para recompilar se build.rs mudar
    println!("cargo:rerun-if-changed=build.rs");
}
```

#### No arquivo `src/main.rs`

```rust
include!(concat!(env!("OUT_DIR"), "/saudacao.rs"));

fn main() {
    println!("{}", saudar());
}
```

> Isto mantém o código gerado na pasta `target/`, **fora da sua pasta `src`**, evitando problemas com controle de versão e assegurando compilações limpas.

Se você rodar `cargo run` verá esse resultado: 

```shell
Olá, Fulano! Este código foi criado em 05/08/2025 19:09:45.
```

Esta mensagem foi criada pela função `saudar()`, gerada pelo `build.rs` e incluída no `main.rs`. 

## Custom Cargo Commands and Scripts

Cargo, o poderoso sistema de build e gerenciador de pacotes do Rust, não é apenas para compilar e testar. Uma das suas funcionalidades mais subestimadas é a extensibilidade: você pode criar comandos customizados para o Cargo utilizando convenções simples de nomenclatura e scripts.

Isso permite que equipes e desenvolvedores automatizem tarefas comuns, como formatação, linting, lançamento de versões ou execução de servidores de desenvolvimento, utilizando uma interface limpa e consistente: `cargo <seu-comando>`.

### Como funcionam os comandos customizados no Cargo

O Cargo procura por executáveis no seu `PATH` que sigam essa convenção de nomenclatura:

```
cargo-<subcomando>
```

Quando você digita no terminal:

```bash
cargo meucomando
```

O Cargo procura por um executável `cargo-meucomando` nos caminhos da variável `PATH`. Se encontrar, ele executa como se você tivesse digitado `cargo-meucomando`. 

Isto significa que você pode escrever comandos em qualquer linguagem, como: **Rust, Bash, Python etc**, desde que o executável siga a convenção de nomenclatura.

#### Um script bash `cargo-sauda`

Crie um comando chamado `cargo sauda` que mostra uma saudação.

1. **Crie o script** (`cargo-sauda` sem extensão):

```bash
#!/bin/bash
echo "Olá, do seu projeto Cargo!"
```

2. Altere as permissões (Linux):

```bash
chmod +x cargo-sauda
```

3. Mova o script para uma pasta dentro do seu `PATH`, por exemplo: `~/.cargo/bin/` (recomendado):

```bash
mv cargo-sauda ~/.cargo/bin/
```

> `~/.cargo/bin` é adicionado ao `PATH` quando você instala Rust com `rustup`.

4. Agora, execute:

```bash
cargo sauda
Olá, do seu projeto Cargo!
```
#### Um binário Rust para validar projetos `cargo-checklist`

Vamos criar uma ferramenta mais avançada para validar as coisas mais comuns em qualquer projeto.

1. Crie um novo projeto binário Rust:

```bash
cargo new cargo-checklist --bin
cd cargo-checklist
```

2. Substitua o `src/main.rs`:

```rust
use std::process::Command;

fn main() {
    println!("Executando a validação do projeto...\n");

    // Verifica a formatação do código
    let fmt = Command::new("cargo").args(["fmt", "--check"]).output().unwrap();
    if fmt.status.success() {
        println!("OK - Código formatado corretamente");
    } else {
        println!("ERRO - Código não formatado corretamente");
    }

    // Executa o Clippy para verificar problemas de lint
    let clippy = Command::new("cargo").args(["clippy", "--all-targets", "--", "-D", "warnings"]).output().unwrap();
    if clippy.status.success() {
        println!("OK - O Clippy não encontrou problemas");
    } else {
        println!("ERRO - O Clippy encontrou problemas");
    }

    // Executa os testes do projeto
    let test = Command::new("cargo").arg("test").output().unwrap();
    if test.status.success() {
        println!("OK - Todos os testes passaram");
    } else {
        println!("ERRO - Alguns testes falharam");
    }

    println!("\nFim da validação!");
}
```

E verifique o `Cargo.toml`: 

```toml
[package]
name = "cargo-checklist"
version = "0.1.0"
edition = "2021"
description = "Uma ferramenta para executar validações: fmt, clippy e tests"
repository = "https://github.com/yourusername/cargo-checklist"

[dependencies]
```

> **Atenção:** O nome do projeto (package name) **DEVE** ser `cargo-checklist`! Se utilizar outro `package name` então terá que renomear o executável criando uma seção do tipo array `[[bin]]` com os atributos `name` e `path`. 

> **Seções em forma de array**: No TOML, `[[bin]]` (com colchetes duplos) denota um **array de tabelas** em vez de uma única tabela. Isso permite declarar múltiplos binários no mesmo `Cargo.toml`, cada um como um elemento separado do array `bin`. Se você usasse `[bin]` (com colchetes simples), obteria apenas uma única tabela chamada `bin`, e as declarações subsequentes sobrescreveriam a anterior. Portanto, sempre que quiser listar mais de um executável (ou mesmo apenas um, seguindo o padrão do Cargo), você deve usar `[[bin]]`. O Cargo iterará sobre cada item desse array e gerará um binário para cada um.

3. Compilar e instalar:

```bash
cargo install --path .
```

Este comando compila e instala o executável `cargo-checklist` na pasta `~/.cargo/bin`.

4. Use em qualquer raiz de projeto Rust:

```bash
cargo checklist
```

Exemplo de saída:
```
cargo checklist
Executando a validação do projeto...

OK - Código formatado corretamente
OK - O Clippy não encontrou problemas
OK - Todos os testes passaram

Fim da validação!
```

### Utilizando scripts apenas no seu projeto (comandos locais)

Em certos casos, talvez você não queira instalar os comandos de forma global. Neste caso, você pode criar scripts locais utilizando uma pasta `scripts/` e um código wrapper.

Exemplo: `scripts/cargo-dev-server`

```bash
#!/bin/bash
echo "Starting dev server..."
cargo watch -x "run"
```

Torne-o executável:

```bash
./scripts/cargo-dev-server
```

> Dica: Use `just` (https://github.com/casey/just) ou `cargo-make` para executar tarefas de forma mais robusta.

### Comandos que usam este padrão

Alguns comandos Cargo populares foram implementados assim:

- `cargo fmt` → `rustfmt` (via `cargo-fmt`)
- `cargo clippy` → `clippy` (via `cargo-clippy`)
- `cargo watch` → external binary
- `cargo sqlx` → `cargo-sqlx`

Você pode até listar os comandos Cargo disponíveis:

```bash
cargo --list
```

Output:
```
Available commands:
    build          Compile a local package and all of its dependencies
    check          Check a local package and all of its dependencies for errors
    clean          Remove artifacts that cargo has generated in the past
    ...
    greet          (from ~/.cargo/bin/cargo-greet)
    checklist      (from ~/.cargo/bin/cargo-checklist)
```

### Boas práticas

- Use sempre a nomenclatura correta `cargo-<command>`.
- Grave seus executáveis dentro de `~/.cargo/bin`.
- Sempre use `cargo install --path .` para comandos.
- Trate os erros e forneça sempre mensagens significativas.
- Considere compatibilidade entre plataformas.

