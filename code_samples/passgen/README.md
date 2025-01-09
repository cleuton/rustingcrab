![](rusting-crab-logo.png)

[**Cleuton Sampaio**](https://linkedin.com/in/cleutonsampaio) - Me siga!
[**RustingCrab.com**](https://rustingcrab.com).

[**Path no GitHub:**]()

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

