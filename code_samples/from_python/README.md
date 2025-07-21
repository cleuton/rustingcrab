<img src="./from_python.png" height=300>

# From Python with love

<img src="../../rusting-crab-logo.png" height=300>

[**Cleuton Sampaio**](https://linkedin.com/in/cleutonsampaio)

[**Veja no GitHub**](https://https://github.com/cleuton/rustingcrab/tree/main/code_samples/from_python)

Fui contratado para resolver um problema de performance na API de um cliente, toda feita em **Python**. Eles queriam reescrever tudo em **Golang**, mas acabei convencendo-os de que seria demais e que dava para separar apenas uma pequena parte do código, que era mais lenta, e transformar em libs externas. Inicialmente, pensei em **C++**, mas depois me lembrei do **Rust**. Depois de muito pesquisar, descobri o `pyo3`, que faz esse "meio de campo" entre o Python e o Rust. 

Deu super certo e combina com o livro que estou lendo: **Refactoring to Rust**, que mostra uma abordagem incremental para adicionar Rust aos seus projetos, sem ter que refazer tudo.

## Gerador snowflake

GeradorSnowflake é uma pequena biblioteca que traz para o seu Python um “ID Snowflake” de 64 bits — um número único que carrega informação de data e ordem, ideal para gerar identificadores sem precisar de banco ou de sequência centralizada.

Você só precisa ter Python 3.12 no seu ambiente e o maturin instalado (veja o Requirements.txt). Depois, dentro da pasta do projeto (onde estão Cargo.toml, lib.rs e teste.py), instale a extensão Rust/Python com um único comando:

```bash
pip install --upgrade maturin
maturin develop --release
```

Isso vai compilar o Rust, gerar a biblioteca nativa e instalá‑la no seu virtualenv. A partir daí, basta criar um script Python como o teste.py:

```python
from gerador_snowflake import GeradorSnowflake

gerador = GeradorSnowflake(1)    # “1” é o número do seu “worker” ou processo
print(gerador.proximo_id())      # exibe um ID único
```

Por baixo dos panos, o Rust mantém três partes num único inteiro de 64 bits:
- um carimbo de tempo (milissegundos desde 1 de jan 2020)
- um identificador de “trabalhador” (até 10 bits)
- uma sequência que evita colisões quando você gera vários IDs no mesmo milissegundo

Se o relógio do sistema retroceder, ou se você tentar gerar IDs tão rápido que a sequência estoure, o gerador trata esses casos com espera ou com erro, para garantir que cada ID seja sempre único e monotonicamente crescente.

Em resumo, com esse projeto você ganha no Python a performance e segurança de tipos do Rust, sem lidar diretamente com ponteiros nem com bindings C. Basta compilar com maturin e chamar `proximo_id()` no seu código Python.

## Examinando o código

A geração de um ID Snowflake segue três passos básicos: medir o tempo desde uma “época” fixa, reservar alguns bits para identificar a máquina (ou processo) que está gerando, e usar uma pequena sequência para evitar colisões quando você pedir vários IDs no mesmo milissegundo. No nosso Rust, tudo isso fica assim:

```rust
const EPOCA: u64          = 1_577_836_800_000;      // início: 1 jan 2020 em ms
const BITS_ID: u8         = 10;                     // até 1024 “trabalhadores”
const BITS_SEQ: u8        = 12;                     // até 4096 IDs/milissegundo
const MAX_ID: u64         = (1 << BITS_ID) - 1;
const MAX_SEQ: u64        = (1 << BITS_SEQ) - 1;
const SHIFT_ID: u8        = BITS_SEQ;
const SHIFT_TIMESTAMP: u8 = BITS_SEQ + BITS_ID;
```

essas constantes definem quantos bits vamos usar para cada parte do ID de 64 bits: primeiro vêm (timestamp − EPOCA), depois o `id_trab` e por fim a `seq`. No struct, guardamos o último timestamp usado e a sequência corrente:

```rust
#[pyclass]
pub struct GeradorSnowflake {
    id_trab: u64,
    seq:     u64,
    last_ts: u64,
}
```

o método marcado com `#[new]` valida o `id_trab` para garantir que caiba nos 10 bits. em `proximo_id` nós

1. pegamos o tempo atual em milissegundos (`SystemTime::now().duration_since(UNIX_EPOCH)… as_millis()`)
2. se o relógio andou para trás, retornamos erro para evitar IDs duplicados;
3. se for o mesmo milissegundo do último, incrementamos `seq` e, ao “estourar” a sequência, aguardamos até o relógio avançar;
4. se for um milissegundo novo, zeramos a sequência;
5. finalmente montamos o ID bit a bit: deslocamos `(ts − EPOCA)` por `SHIFT_TIMESTAMP`, adicionamos `id_trab << SHIFT_ID` e combinamos com `seq`.

Para expor isso ao Python usando PyO3, você anota o struct com `#[pyclass]` e envolve seus métodos em um bloco `#[pymethods]`. o `#[new]` declara o construtor, e qualquer método que retorne `PyResult<T>` vira um método Python. por fim o `#[pymodule]` cria a função de inicialização do módulo:

```rust
#[pymodule]
fn gerador_snowflake(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<GeradorSnowflake>()?;
    Ok(())
}
```

nesse init você recebe o módulo já vinculado ao GIL como um `Bound<'_, PyModule>`, e chama `m.add_class` para registrar `GeradorSnowflake` como uma classe Python. depois de compilar com `maturin develop --release`, basta no Python:

```python
from gerador_snowflake import GeradorSnowflake

g = GeradorSnowflake(1)
print(g.proximo_id())
```



