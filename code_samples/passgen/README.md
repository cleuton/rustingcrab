![](rusting-crab-logo.png)

[**Cleuton Sampaio**](https://linkedin.com/in/cleutonsampaio) - Me siga!
[**RustingCrab.com**](https://rustingcrab.com).

[**Path no GitHub:**](https://github.com/cleuton/rustingcrab/tree/main/code_samples/passgen)

# Bora gerar senhas?

Dev, gerar senhas é algo muito importante hoje em dia. Sei que existem vários softwares geradores de senhas por aí, mas esse aqui é em **Rust**!

## Como usar o projeto

Para gerar uma senha e exibir no terminal: 

```shell
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/passgen`
Senha gerada: T_#Wc7(S04u#
```

Para gerar uma senha e salvar no arquivo `config.toml` com uma `url`: 

```shell
$ cargo run aws
``` 

Ele vai criar uma chave "aws" com a senha gerada. Eis o arquivo `config.toml`: 

```toml
[regra]
caracteres_especiais = true
maiusculas = true
minusculas = true
pelo_menos_um_caracter_especial = true
pelo_menos_um_digito = true
pelo_menos_uma_letra_maiuscula = true
tamanho_senha = 12

[senhas]
aws = "2,ae/3F6T.4O"
```

Cada URL que você passar ele vai incluir na tabela `[senhas]` ou atualizar o valor, de acordo com as regras: 

- caracteres_especiais: Se deve incluir caracteres especiais (true | false)
- maiusculas: Se deve incluir letras maiúsculas (true | false)
- minusculas: Se deve incluir letras minúsculas (true | false)
- pelo_menos_um_caracter_especial:  Se deve incluir pelo menos 1 caractere especial (true | false)
- pelo_menos_um_digito: Se deve incluir pelo menos 1 dígito (true | false)
- pelo_menos_uma_letra_maiuscula: Se deve incluir pelo menos 1 letra minúscula (true | false)
- tamanho_senha: Comprimento da senha em caracteres

Se o arquivo `config.toml` não existir, ele o criará com regra default. 

## Estrutura do projeto

Este projeto é um **gerador de senhas personalizável** que utiliza configurações definidas em um arquivo TOML (`config.toml`). Ele suporta:

1. **Geração de senhas**: Baseando-se em regras especificadas, como uso de letras maiúsculas, minúsculas, caracteres especiais e dígitos.
2. **Persistência**: Permite salvar senhas associadas a URLs no arquivo TOML na tabela `[senhas]`.
3. **Configuração automática**: Se o arquivo `config.toml` não existir, ele é criado com valores padrão.

### Estrutura

```
passgen
│
├── src
│   ├── main.rs               // Ponto de entrada principal
│   ├── modulos
│   │   ├── mod.rs            // Gerencia os módulos
│   │   └── toml_proc.rs      // Funções para manipulação do arquivo TOML
│
├── Cargo.toml                // Configuração do projeto e dependências
└── config.toml               // Arquivo de configuração gerado ou manipulado
```

### Componentes

1. **`main.rs`**:
   - Função principal (`main`):
     - Carrega as regras de geração de senha.
     - Gera a senha com base nas regras.
     - Salva a senha no arquivo TOML se um argumento (URL) for passado via linha de comando.
   - Função `gerar_senha`:
     - Implementa a lógica de geração de senhas, garantindo a randomização dos caracteres e cumprimento das regras.

2. **`modulos/mod.rs`**:
   - Ativa o módulo `toml_proc` para centralizar o código relacionado à manipulação de TOML.

3. **`modulos/toml_proc.rs`**:
   - **`Regra`**: Struct que define os critérios de geração de senhas.
   - **`ler_regra`**: Lê as regras de `[regra]` no TOML ou cria um arquivo padrão se não existir.
   - **`salvar_senha`**: Adiciona ou atualiza senhas associadas a URLs na tabela `[senhas]`.

4. **`config.toml`** (gerado automaticamente):
   - Contém configurações e senhas salvas.
   - Estrutura típica:
     ```toml
     [regra]
     tamanho_senha = 12
     maiusculas = true
     minusculas = true
     caracteres_especiais = true
     pelo_menos_um_digito = true
     pelo_menos_um_caracter_especial = true
     pelo_menos_uma_letra_maiuscula = true

     [senhas]
     "https://example.com" = "senha123"
     ```

### Motivação para a Estrutura

1. **Separação de responsabilidades**:
   - `main.rs`: Foco na lógica principal e no fluxo do programa.
   - `modulos/toml_proc.rs`: Gerencia manipulação de TOML e lógica de regras.

2. **Reutilização e modularidade**:
   - A lógica de manipulação do TOML (`ler_regra`, `salvar_senha`) pode ser reutilizada ou testada isoladamente.

3. **Clareza e extensibilidade**:
   - A divisão em módulos facilita a manutenção e a adição de funcionalidades futuras, como novos critérios para geração de senhas.

4. **Automação de configuração**:
   - A geração automática de `config.toml` torna o programa mais amigável, permitindo que ele funcione "pronto para uso".

### Processador de arquivo Toml

**src/modulos/toml_proc.rs:**

```rust
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use toml::{map::Map, Value};

#[derive(Debug)]
pub struct Regra {
    pub tamanho_senha: i32,
    pub maiusculas: bool,
    pub minusculas: bool,
    pub caracteres_especiais: bool,
    pub pelo_menos_um_digito: bool,
    pub pelo_menos_um_caracter_especial: bool,
    pub pelo_menos_uma_letra_maiuscula: bool,
}

impl Regra {
    // Função para construir uma instância de Regra a partir de um Value do TOML
    pub fn from_toml(value: &Value) -> Result<Self, io::Error> {
        let tabela = value.as_table().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "Tabela 'regra' inválida ou ausente")
        })?;

        Ok(Self {
            tamanho_senha: tabela.get("tamanho_senha")
                .and_then(|v| v.as_integer())
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Campo 'tamanho_senha' inválido ou ausente"))? as i32,
            maiusculas: tabela.get("maiusculas")
                .and_then(|v| v.as_bool())
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Campo 'maiusculas' inválido ou ausente"))?,
            minusculas: tabela.get("minusculas")
                .and_then(|v| v.as_bool())
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Campo 'minusculas' inválido ou ausente"))?,
            caracteres_especiais: tabela.get("caracteres_especiais")
                .and_then(|v| v.as_bool())
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Campo 'caracteres_especiais' inválido ou ausente"))?,
            pelo_menos_um_digito: tabela.get("pelo_menos_um_digito")
                .and_then(|v| v.as_bool())
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Campo 'pelo_menos_um_digito' inválido ou ausente"))?,
            pelo_menos_um_caracter_especial: tabela.get("pelo_menos_um_caracter_especial")
                .and_then(|v| v.as_bool())
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Campo 'pelo_menos_um_caracter_especial' inválido ou ausente"))?,
            pelo_menos_uma_letra_maiuscula: tabela.get("pelo_menos_uma_letra_maiuscula")
                .and_then(|v| v.as_bool())
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Campo 'pelo_menos_uma_letra_maiuscula' inválido ou ausente"))?,
        })
    }
}

pub fn salvar_senha(url: &str, senha: &str) -> io::Result<()> {
    let caminho = "config.toml";
    let conteudo = fs::read_to_string(caminho)?;
    let mut config: Value = conteudo.parse::<Value>()?;

    // Pega a tabela raiz do arquivo TOML
    let config_table = config.as_table_mut().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidData, "Formato inválido no arquivo TOML")
    })?;

    // Verifica se a tabela [senhas] existe
    if let Some(tabela) = config_table.get_mut("senhas") {
        // A tabela já existe, atualiza com os valores padrão
        if let Some(tabela_map) = tabela.as_table_mut() {
            tabela_map.insert(url.to_string(), Value::String(senha.to_string()));
        }
    } else {
        // A tabela não existe, cria uma nova tabela com os valores padrão
        let mut valores_padrao = Map::new();
        valores_padrao.insert(url.to_string(), Value::String(senha.to_string()));
        config_table.insert("senhas".to_string(), Value::Table(valores_padrao));
    }

    // Escreve o arquivo atualizado
    let conteudo_atualizado = toml::to_string(&config).expect("Erro ao converter para TOML");
    let mut arquivo = File::create(caminho)?;
    arquivo.write_all(conteudo_atualizado.as_bytes())?;

    Ok(())

}

pub fn ler_regra() -> io::Result<Regra> {
    let caminho = "config.toml";

    // 1. Verificar se o arquivo existe
    if !Path::new(caminho).exists() {
        println!("Arquivo config não encontrado. Criando com valores padrão...");

        let regra_def = Regra {
            tamanho_senha: 12,
            maiusculas: true,
            minusculas: true,
            caracteres_especiais: true,
            pelo_menos_um_digito: true,
            pelo_menos_um_caracter_especial: true,
            pelo_menos_uma_letra_maiuscula: true,
        };

        // Criar valores padrão para o arquivo TOML
        let mut valores_padrao = Map::new();
        let mut regra = Map::new();
        regra.insert("tamanho_senha".to_string(), Value::Integer(regra_def.tamanho_senha.into()));
        regra.insert("maiusculas".to_string(), Value::Boolean(regra_def.maiusculas));
        regra.insert("minusculas".to_string(), Value::Boolean(regra_def.minusculas));
        regra.insert("caracteres_especiais".to_string(), Value::Boolean(regra_def.caracteres_especiais));
        regra.insert("pelo_menos_um_digito".to_string(), Value::Boolean(regra_def.pelo_menos_um_digito));
        regra.insert("pelo_menos_um_caracter_especial".to_string(), Value::Boolean(regra_def.pelo_menos_um_caracter_especial));
        regra.insert("pelo_menos_uma_letra_maiuscula".to_string(), Value::Boolean(regra_def.pelo_menos_uma_letra_maiuscula));
        valores_padrao.insert("regra".to_string(), Value::Table(regra));

        // Converter para string TOML
        let conteudo_padrao = toml::to_string(&Value::Table(valores_padrao))
            .expect("Erro ao converter valores padrão para TOML");

        // Criar o arquivo e escrever os valores padrão
        let mut arquivo = File::create(caminho)?;
        arquivo.write_all(conteudo_padrao.as_bytes())?;

        println!("Arquivo config criado com sucesso!");
        return Ok(regra_def);
    }

    // 2. Ler o arquivo TOML existente
    let conteudo = fs::read_to_string(caminho)?;
    let config: Value = conteudo.parse::<Value>().expect("Erro ao analisar o TOML");

    // 3. Extrair a tabela [regra] e convertê-la para a struct Regra
    let regra = config.get("regra")
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Tabela 'regra' não encontrada"))?;

    Regra::from_toml(regra)
}
```

### Main e função de geração de senha

**src/main.rs:**

```rust
mod modulos;
use std::env;
use std::io::{self};
use rand::seq::SliceRandom;
use rand::thread_rng;
use modulos::toml_proc::{Regra,ler_regra,salvar_senha};

pub fn gerar_senha(regra: &Regra) -> String {
    let mut rng = thread_rng();

    // Conjuntos de caracteres para cada regra
    let minusculas: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let maiusculas: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let digitos: Vec<char> = "0123456789".chars().collect();
    let especiais: Vec<char> = "!@#$%^&*()-_=+[]{}|;:,.<>?/".chars().collect();

    // Vetor para armazenar o pool de caracteres
    let mut pool: Vec<char> = Vec::new();

    // Adiciona ao pool de caracteres de acordo com as regras
    if regra.minusculas {
        pool.extend(&minusculas);
    }
    if regra.maiusculas {
        pool.extend(&maiusculas);
    }
    if regra.caracteres_especiais {
        pool.extend(&especiais);
    }
    if regra.pelo_menos_um_digito {
        pool.extend(&digitos);
    }

    // Vetor para armazenar a senha
    let mut senha_chars: Vec<char> = Vec::new();

    // Garante os requisitos obrigatórios
    if regra.pelo_menos_uma_letra_maiuscula {
        senha_chars.push(*maiusculas.choose(&mut rng).unwrap());
    }
    if regra.pelo_menos_um_caracter_especial {
        senha_chars.push(*especiais.choose(&mut rng).unwrap());
    }
    if regra.pelo_menos_um_digito {
        senha_chars.push(*digitos.choose(&mut rng).unwrap());
    }

    // Gera os caracteres restantes para completar o tamanho da senha
    let restante = (regra.tamanho_senha as usize).saturating_sub(senha_chars.len());
    for _ in 0..restante {
        senha_chars.push(*pool.choose(&mut rng).unwrap());
    }

    // Embaralha todos os caracteres da senha
    senha_chars.shuffle(&mut rng);

    senha_chars.into_iter().collect()
}

fn main() -> io::Result<()> {
    let regra = ler_regra()?;
    
    let senha = gerar_senha(&regra);

    // Lê os argumentos da linha de comando
    let args: Vec<String> = env::args().collect();

    // Verifica se um argumento foi passado
    if args.len() > 1 {
        let url = &args[1];
        salvar_senha(&url, &senha)?;
    } else {
        println!("Senha gerada: {}", senha);
    }

    Ok(())
}
```