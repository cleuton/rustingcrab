<img src="de_go_para_rust.png" height=400>

---

<img src="../../rusting-crab-logo.png" height=300>

---

[**Cleuton Sampaio**](https://linkedin.com/in/cleutonsampaio)

[**Veja no GitHub**](https://https://github.com/cleuton/rustingcrab/tree/main/code_samples/rust_loves_go)

# De Go para Rust com amor

Já pensou em integrar módulos **Rust** às suas aplicações em **Go**? Quais as vantagens? Bom...

1. **Desempenho sem GC**
   Rust não tem coletor de lixo. Em trechos críticos (cripto, compressão, parsers binários, imagem/vídeo, simd), você ganha throughput e latência mais previsível que em Go, útil quando pausas do GC incomodam.

2. **Segurança de memória melhor que C via cgo**
   Se você já desceria para C por performance, Rust dá o mesmo nível de baixo nível com verificação de empréstimos, evitando use-after-free, double free e data races. É a opção “rápida e segura” para a fronteira FFI do Go.

3. **Acesso a ecossistema e recursos de baixo nível**
   Crates maduros para codecs, cripto, regex de alta perf., SIMD (via std::arch/portable-simd), zero-copy, io\_uring, bindings de drivers nativos etc. Você reaproveita muito código de sistema existente sem reescrever em Go.

4. **Latência previsível e p99 menor**
   Para pipelines que exigem jitter baixo (trading, media processing, inference local), mover o hot path para Rust reduz caudas (p95/p99), especialmente sob pressão do heap do Go.

5. **Binários simples de distribuir**
   Rust compila bem como staticlib/cdylib. Dá para linkar estaticamente (musl) e simplificar deploy de um único binário Go chamando a lib Rust — útil em edge e containers mínimos.

6. **Concorrência sem data races no trecho crítico**
   Você pode manter a orquestração em Go (goroutines, canais) e executar paralelismo pesado no lado Rust (rayon, tokio para I/O), isolando o risco de corridas onde a performance importa.

7. **Interoperabilidade previsível**
   FFI C estável, assinatura simples (ptr + len, ints, códigos de erro), sem dependência de runtimes exóticos. Facilita versionar e testar.

Em que tipo de apps usar? 

* Criptografia, compressão, codecs (imagem/áudio/vídeo), parsing binário pesado, regex streaming.
* Processamento vetorial/SIMD, DSP, ML light em CPU.
* Motores de regras/score rápido com p99 apertado.
* Qualquer coisa que você faria em C/C++ para ganhar performance — só que com menos riscos.

Nestes casos talvez seja melhor evitar:

* Chamadas FFI muito granulares (milhões de calls pequenos) perdem no overhead de cgo. Prefira chamadas “grossas” que processam lotes.
* Lógica de negócio comum, I/O de rede padrão e ergonomia: geralmente fique em Go.
* Passagem de strings e ownership: defina quem aloca/libera (evite alocar em uma linguagem e liberar na outra).

Boas práticas rápidas:

* API C-friendly: `extern "C"`, tipos primitivos, `ptr + len`, códigos de erro; nunca deixe `panic!` atravessar FFI.
* Memória: quem aloca, libera. Se precisar, expose `free_*` no lado Rust (usando o mesmo allocator).
* Versão do header: gere com `cbindgen` para evitar drift.
* Build: use `staticlib` para facilitar link e distribuição; no Linux, às vezes adicione `-ldl -lm -lpthread`.
* Performance: faça o trabalho pesado em uma chamada só; evite chatice cross-boundary.

## Projeto simples

Esse projeto é extremamente simples, com uma estrutura de diretórios enxuta: 

```text
gorust/
      |
      +-src/lib.rs
      |
      +-Cargo.toml
      |
      +-goapp/
             |
             +-main.go
             |
             +-include/<header C da função Rust>
             |
             +-lib/<bin da lib rust>
```

O código **Rust** é extremamente simples: 

```rust
#[unsafe(no_mangle)]
pub extern "C" fn somar(a: i32, b: i32) -> i32 {
    a + b
}
```

E o `Cargo.toml` também é simples: 

```toml
[package]
name = "gorust"
version = "0.1.0"
edition = "2024"

[lib]
# Vai gerar duas libs: Estática e dinâmica:
crate-type = ["cdylib", "staticlib"]

[dependencies]
libc = "0.2.2"
```

E, do lado `Go` também: 

```go
package main

/*
#cgo CFLAGS:  -I${SRCDIR}/include
#cgo LDFLAGS: -L${SRCDIR}/lib -lgorust -ldl -lm -lpthread
#include "gorust.h"
*/
import "C"
import "fmt"

func main() {
	res := C.somar(C.int(10), C.int(20))
	fmt.Printf("Result from Rust: %d\n", int(res))
}
```

Esses comentários são processados pelo **CGO** para compilar seu código! Note que tem um flag para carregar nossa lib `-lgorust` de forma estática, incorporando tudo ao executável `Go`. Dá para fazer de forma dinâmica também, mas você terá que distribuir arquivos separados.

## Como compilar

Para começar, compile o código `Rust` com: `cargo build`.

O **CGO** precisa da assinatura da função `Rust` convertida em `C`. Se você quiser, pode gerar um `.h` simples ou utilizar o `cbindgen`:

```shell
cargo install --force cbindgen
cbindgen --lang c --output ./goapp/include/gorust.h
```

Isto vai instalar o `cbindgen` e gerar um `.h` diretamente na subpasta `include` do projeto `Go`.

Uma alternativa é anotar o código `Go` com a assinatura da função, mas você vai ter que saber `C`: 

```go
package main

/*
extern int somar(int a, int b);
#cgo LDFLAGS: -L${SRCDIR}/lib -lgorust -ldl -lm -lpthread
*/
```

Eu fui testar isso e deram alguns erros, então preferi voltar à primeira forma.

Agora é só compilar o código `Go`: 

```shell
cp ./target/debug/libgorust.a ./goapp/lib
cd goapp
go mod init gorust
export CGO_ENABLED=1
go build
```
E executar: 

```shell
./gorust
Result from Rust: 30
```
