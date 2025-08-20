<img src="Solana_logo.png" height=300>

---

<img src="../../rusting-crab-logo.png" height=300>

---


[**Cleuton Sampaio**](https://linkedin.com/in/cleutonsampaio)

[**Veja no GitHub**](https://https://github.com/cleuton/rustingcrab/tree/main/code_samples/solana-first-steps)


# Solana first steps

A tecnologia de **blockchain** é fascinante, e a rede **Solana** é um dos blockchains mais interessantes do momento, até porque usa **Rust** em sua **API**. 

Vamos começar devagar para você entender. Teremos uma série de tutoriais sobre Solana para você e esse é só o primeiro. 

## Blockchain em poucas linhas

* **Blockchain** é um banco de dados distribuído. Em vez de ficar em um servidor central, a cópia está espalhada por milhares de computadores (nós).
* Os dados são gravados em **blocos**, que são encadeados (chain) uns aos outros. Cada bloco contém transações e um “hash” que garante que nada foi alterado.
* O sistema funciona de forma **imutável**: uma vez gravado em bloco, não pode ser alterado sem reescrever toda a cadeia.
* Quem mantém a rede são os **validadores** (ou mineradores em outras redes). Eles checam transações, produzem blocos e garantem o consenso.
* Para usar a rede, você precisa de uma **carteira** (wallet), que tem um par de chaves criptográficas:

  * **chave pública** (endereço da conta)
  * **chave privada** (assinatura das transações)

---

## Solana em termos simples

Solana é uma blockchain pública de alta performance. O objetivo dela é ser **rápida e barata** para rodar programas (smart contracts).

* **Contas**: tudo na Solana é uma conta. Até o próprio programa é uma conta. As contas armazenam dados (lamports, tokens, estado do programa).
* **Programas**: são smart contracts escritos em Rust, C ou C++. Eles rodam dentro da rede, mas são imutáveis após o deploy.
* **Instruções**: para “invocar” um programa, você envia uma transação contendo instruções.
* **Transações**: agrupam instruções e são assinadas pela sua wallet. Uma vez confirmada, a rede executa e grava o resultado.
* **Lamports**: a menor unidade do SOL (1 SOL = 1 bilhão de lamports). Usado para pagar taxas de transação e armazenar dados on-chain.
* **Devnet / Testnet / Mainnet**:

  * **Devnet** é a rede de testes gratuita, onde você pega SOL de brincadeira em faucets.
  * **Testnet** é usada para simular performance em larga escala.
  * **Mainnet** é a rede real, onde o SOL tem valor econômico.

---

## Fluxo prático na Solana

1. **Gerar uma wallet** com `solana-keygen new`.
2. **Pegar SOL de teste** com `solana airdrop 2` (na devnet).
3. **Escrever o programa** em Rust e compilar para BPF (`cargo build-bpf`).
4. **Deployar** com `solana program deploy target/deploy/meu_programa.so`.
5. **Invocar** criando uma transação (via web3.js, Anchor, ou teste em Rust) que chama o `program_id`.
6. **Ver logs** com `solana confirm <assinatura> --verbose` ou `solana logs <program_id>`.

---

## O que você precisa guardar

* Blockchain = banco distribuído imutável.
* Solana = blockchain rápida e barata, onde tudo são contas e programas.
* Para interagir, você **sempre** precisa de:

  1. uma carteira (com SOL para taxas),
  2. um programa (caso de smart contract),
  3. e uma transação (que invoca instruções).

## Instalando o kit solana

Você já tem o **Rust** instalado, certo? Caso contrário, use o script [**rustup**](https://www.rust-lang.org/tools/install) para instalar o `toolchain` do **Rust**, que inclui o `cargo`. 

Para isso temos que considerar o seu sistema operacional.

## MacOS ou Linux

Simplesmente baixe e execute o `rustup`: 

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Este script instala todo o `toolchain` dentro da sua pasta `home/.cargo/bin`. Ele tentará ajustar a variável `PATH` Para incluir essa pasta. Após a instalação, tente usar o comando `rustc --version` se ele não for encontrado, é só adicionar a pasta `~/.cargo/bin` à variável `PATH`, o que pode ser feito no seu script de login. 

```bat
rustc --version
cargo --version
```

Se aparecerem as versões do Rust e do Cargo, a instalação foi bem-sucedida. A partir daí, você já pode criar projetos com `cargo new` e compilar normalmente.

> Sobre **MS Windows:** É possível instalar **Rust** e até **Solana** no **Windows**. Muita gente usa o **WSL2** para isso. Eu não tenho Windows e não tenho como testar os comandos, por isso só mostrei **linux / macos**.

### Agora vamos ao resto...

#### 1) Instalação rápida (CLI, Rust, Anchor, Node/Yarn)

Instala o ecossistema de desenvolvimento (útil mesmo que você não vá usar Anchor agora).

```bash
curl --proto '=https' --tlsv1.2 -sSfL https://solana-install.solana.workers.dev | bash
```

O script baixa e configura a Solana CLI (Agave), instala/atualiza Rust e ferramentas de suporte.

#### 2) PATH permanente

Garante que os binários da Solana estejam sempre disponíveis no terminal.

```bash
echo 'export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"' >> ~/.bashrc
echo 'export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"' >> ~/.profile
source ~/.bashrc
solana --version
```

#### 3) Apontar para a Devnet

Usaremos a rede de testes gratuita.

```bash
solana config set -ud      # equivale a --url https://api.devnet.solana.com
solana config get
```

A `Devnet` é uma rede `solana` para desenvolvimento e você pode usar tokens de teste. 

#### 4) Criar uma wallet local (para assinar e pagar taxas na Devnet)

```bash
solana-keygen new          # cria ~/.config/solana/id.json
solana address             # mostra a public key
```

#### 5) Obter SOL de teste (faucet)

Moedas de teste para pagar deploy/tx na Devnet.

```bash
solana airdrop 2
solana balance
```

Acredite: Para estudar `solana` 2 SOL é mais que suficiente. Se precisar de mais, rode o `airdrop 2` novamente (só permite 2 a cada vez e tem um rate-limit).

> **Atenção:** A `Devnet` costuma dar muito **timeout**, então, caso aconteça em algum comando, tente novamente.

#### 7) Ferramenta de build on-chain

Hoje o fluxo recomendado para compilar seu código **Rust** gera o `bytecode` **Solana BPF**, utilizando cargo `build-sbf` (vem junto com a CLI moderna). Para verificar se você tem o `build-sbf`: 

```bash
cargo --list | grep build-sbf || echo "Se não aparecer, atualize a Solana CLI."
```

Se não existir, atualize a CLI com o próprio instalador novamente (ou use solana-install update/agave-install update, conforme o que veio no seu ambiente).

## Um programa on-chain

Todo programa que execute no `blockchain` `solana` é um **programa on-chain da Solana**, sendo que alguns usam o termo equivalente ao do **Ethereum** chamam de **smart contract**.

### O que é um smart contract?

* Ele é escrito em Rust usando o crate `solana_program`, que fornece as APIs da runtime.
* O `entrypoint!(process_instruction)` registra a função que será chamada sempre que alguém enviar uma transação invocando esse `program_id`.
* Ele recebe:
  * o `program_id` (identificador único na rede),
  * as `accounts` (as contas que a instrução passa),
  * e os `instruction_data` (dados binários da instrução).
* A função retorna um `ProgramResult`, que é `Ok(())` quando não há erro.

### Diferença para um app comum

Um programa desses não roda no seu computador, devendo rodar dentro da máquina virtual da Solana, no cluster (devnet, testnet ou mainnet). Ele é imutável depois do deploy: você só pode atualizar o binário redeployando com um novo programa (ou, em alguns casos, com o mesmo `program_id` se tiver upgrade authority).

### O que os smart contracts podem fazer

Os programas on-chain na Solana são como **regras automáticas gravadas dentro da blockchain**. Eles servem para que a rede execute lógica que ninguém pode mudar depois do deploy.

Em linguagem simples:

* Você cria um programa quando precisa que algo seja **confiável, transparente e descentralizado**.
* Ele pode **guardar e manipular dados em contas on-chain** (como saldos, registros, pontuações de um jogo).
* Ele pode **validar regras** (ex: só transferir se certas condições forem atendidas).
* Ele pode **permitir interação entre pessoas sem precisar de intermediário** (ex: uma exchange descentralizada, um sistema de votos, um jogo).

Resumindo: a gente cria programas on-chain quando precisa que a própria rede garanta a execução das regras, sem depender de servidor ou banco de dados privado. É como ter um “árbitro automático” que todos confiam porque está rodando dentro da blockchain.

Vejamos três exemplos práticos de **smart contracts**:

**1. Financeiro (DeFi):**
Imagine um “cofre digital” onde várias pessoas depositam tokens e só podem sacar se cumprirem certas condições (por exemplo, emprestar com garantia ou resgatar depois de um prazo). O programa on-chain garante que ninguém pode mexer no dinheiro fora das regras.

**2. Jogo:**
Pense em um joguinho de cartas na blockchain. O programa on-chain pode guardar as cartas de cada jogador em contas e validar as regras da partida. Assim, mesmo que você não confie no servidor do outro jogador, a blockchain garante que o jogo está sendo jogado corretamente.

**3. Registro de dados:**
Você pode criar um programa que guarda certificados, diplomas ou documentos importantes. Uma vez gravados, ninguém pode adulterar. Então, se alguém mostrar um diploma registrado no programa, qualquer um pode conferir na blockchain se é verdadeiro.

Em todos os casos, a ideia é a mesma: você usa um programa on-chain quando precisa que **a regra não dependa de uma empresa ou servidor**, mas sim da própria rede, que todos confiam.

### E como seria um "hello world"?

Ok, vamos criar um `hello-world` simples para demonstrar programas **on-chain**. Neste momento, é melhor entender e exercitar só isso e depois mostraremos usos mais avançados.

Exemplo: 

```rust
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Hello, Solana!");
    Ok(())
}
```

Este código do [**projeto do repositório**](./program/src/lib.rs) é um **programa on-chain** ou **smart contract**. Ele nada faz, apenas gera uma mensagem no log do Solana. 

Esse código é o exemplo mais simples possível de um **programa on-chain da Solana**.

Ele é escrito em Rust e segue o formato que a blockchain espera: define um **entrypoint** (`process_instruction`) que será chamado toda vez que alguém enviar uma transação para esse programa. Essa função recebe:

* o **program\_id** (a identidade do programa na rede),
* as **contas** que a transação passa,
* e os **dados da instrução** (bytes que poderiam ser usados como parâmetros).

Neste caso, o programa não usa nada disso. Ele apenas imprime nos logs a mensagem **“Hello, Solana!”** sempre que for invocado.

Resumindo: é um **“Hello World” na blockchain da Solana** — serve só para mostrar a estrutura mínima de um smart contract em Rust, pronto para depois ser expandido e guardar dados ou aplicar regras de verdade.

#### 1. O `use` - O que deve ser trazido para o escopo

```rust
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};
```

Aqui você está colocando no escopo os utilitários da biblioteca oficial `solana_program`:

* **`Pubkey`** → representa uma chave pública (endereço) dentro da Solana. Cada conta e programa tem um `Pubkey`.
* **`AccountInfo`** → representa uma conta passada para o programa. Uma conta pode guardar SOL, tokens ou até o estado de outro contrato.
* **`ProgramResult`** → é só um tipo padrão (`Result<(), ProgramError>`) que indica se a execução deu certo ou retornou erro.
* **`entrypoint`** → macro que registra a função principal do seu programa. Sem ela, a runtime da Solana não saberia onde começar.
* **`msg!`** → macro que imprime mensagens nos logs da transação (muito usado para debug).

#### 2. O `entrypoint!`

```rust
entrypoint!(process_instruction);
```

Isso diz: “quando alguém chamar esse programa na blockchain, execute a função `process_instruction`”.


#### 3. A função `process_instruction`

```rust
pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Hello, Solana!");
    Ok(())
}
```

* **`_program_id: &Pubkey`**
  É o endereço único do programa na Solana. Ele serve para o programa se identificar e, em alguns casos, verificar se as contas passadas realmente pertencem a ele.

* **`_accounts: &[AccountInfo]`**
  É a lista de contas que a transação envia para o programa. Se fosse um contrato de jogo, por exemplo, poderia ter a conta do jogador, a conta do placar, etc. O underline (`_`) indica que não estamos usando esse argumento.

* **`_instruction_data: &[u8]`**
  São os dados brutos da instrução. Por exemplo, se você quiser mandar “incrementar contador em +1”, esse comando viria aqui em formato de bytes.

* **Retorno: `ProgramResult`**
  Retorna `Ok(())` se tudo deu certo, ou um erro (`Err(ProgramError::... )`) se algo falhar.

#### O teste

Para invocar um programa on-chain na Solana você precisa enviar uma transação para a rede, contendo uma instrução que aponta para o `program_id` e inclui as contas e dados que o programa espera. Essa transação é assinada pela sua carteira e submetida via cliente, que pode ser a Solana CLI para programas nativos como transferências e stake, ou bibliotecas como `@solana/web3.js`, Anchor ou até mesmo um teste em Rust para programas customizados. Depois de confirmada, a rede executa a função `process_instruction` do programa, e você consegue ver o resultado ou as mensagens emitidas nos logs da transação.

Vamos testar o programa em `Rust`. A ideia é compilar esse código para `sbf` e instalar na rede `Devnet` da `solana` e depois invocar um teste para rodar lá. Esse é o [**código de teste**](./program/tests/devnet-invoke.rs):

```rust
use std::str::FromStr;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair, Signer},
    transaction::Transaction,
};

#[test]
#[ignore] // execute manualmente quando quiser chamar a devnet
fn invoca_na_devnet() {
    // usa a chave padrão do solana-cli (~/.config/solana/id.json)
    let payer_path = dirs::home_dir().unwrap().join(".config/solana/id.json");
    let payer: Keypair = read_keypair_file(payer_path).expect("carregar keypair");

    let rpc = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );

    // seu program id
    let program_id =
        Pubkey::from_str("<O PROGRAM_ID ENTRA AQUI>").unwrap();

    // sua instrução não usa contas nem dados
    let ix = Instruction {
        program_id,
        accounts: vec![],
        data: vec![],
    };

    let blockhash = rpc.get_latest_blockhash().expect("pegar blockhash");
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );

    let sig = rpc
        .send_and_confirm_transaction(&tx)
        .expect("enviar transação");
    eprintln!("Tx sig: {}", sig);
}
```

Esse teste é um cliente **off-chain** que conversa com a Devnet via RPC porque, fora da blockchain, a única forma de submeter transações a um cluster Solana é chamando um nó RPC (HTTP/JSON-RPC). 

Ele carrega a chave local que você criou (`~/.config/solana/id.json`) para assinar, cria um `RpcClient` apontando para `https://api.devnet.solana.com` com compromisso `confirmed`, resolve o `Pubkey` do programa (cole o `PROGRAM_ID` que recebeu ao instalar na `Devnet`) e monta uma `Instruction` cujo `program_id` é exatamente o do seu contrato. 

Como a instrução não usa contas nem dados, os vetores vêm vazios; ainda assim, isso é uma instrução válida. 

O cliente pede ao nó RPC o `latest_blockhash` (protege contra replay), constrói uma `Transaction` contendo essa instrução, define o `fee_payer` como sua wallet, assina com a chave e envia usando `send_and_confirm_transaction`. 

O nó RPC propaga a transação para o líder do slot; quando ela é executada, a runtime carrega a conta executável identificada por `program_id` e invoca o `entrypoint!(process_instruction)`, passando as contas e os bytes da instrução. 

Como o programa só faz `msg!("Hello, Solana!")`, você verá esse log ao inspecionar a assinatura retornada. O `#[ignore]` existe para você rodar manualmente (porque depende de rede) e não quebrar sua suíte de testes local.

## Compilando e instalando na Devnet

Para compilar seu programa: 

```bash
cargo build-sbf
```

Se tudo estiver ok, verifique se você tem saldo de tokens `SOL` de teste para fazer deploy na `Devnet`: 

```bash
solana balance
```

Se tiver pelo menos 1 SOL, vai dar para implantar sem problemas. Caso contrário, rode: `solana airdrop 2`.

Depois de compilado, você pode instalar na `Devnet` com esse comando: 

```shell
solana program deploy target/deploy/hello_solana.so
```

Guarde o `PROGRAM_ID` que ele vai te dar! Vai precisar dele para tudo!

Agora, é só testar na `Devnet` com `RPC`: 

```bash
cargo test -q --test devnet-invoke -- --ignored --nocapture
```

**Anote a assinatura da transação da execução** para poder ver o log: 

```bash
solana confirm <ASSINATURA DA TRANSAÇÃO> --verbose

RPC URL: https://api.devnet.solana.com
Default Signer Path: /home/cleuton/.config/solana/id.json
Commitment: confirmed

Transaction executed in slot 402445320:
  Block Time: 2025-08-20T15:36:33-03:00
  Version: legacy
  Recent Blockhash: xxx
  Signature 0: xxx
  Account 0: srw- xxx (fee payer)
  Account 1: -r-x xxx
  Instruction 0
    Program:   xxx (1)
    Data: []
  Status: Ok
    Fee: ◎0.000005
    Account 0 balance: ◎3.86875164 -> ◎3.86874664
    Account 1 balance: ◎0.00114144
  Compute Units Consumed: 211
  Log Messages:
    Program xxx invoke [1]
    Program log: Hello, Solana!
    Program xxx consumed 211 of 200000 compute units
    Program xxx success

Finalized
```

Como pode ver a mensagem `Hello, Solana!` está no log, provando que o **smart contract** foi executado (mesmo sem afetar conta alguma).

## Deletando programas

Se você não quer mais usar o programa, pode removê-lo do `blockchain` com o comando `solana close program <PROGRAM_ID> --bypass-warning`. Mas atenção: Depois que fizer isso não poderá mais invocar o programa! Muito cuidado. Substitua `<PROGRAM_ID>` pelo PROGRAM_ID que recebeu ao instalar o programa na `Devnet`.

```bash
$ solana program close <PROGRAM_ID> --bypass-warning

Closed Program Id <PROGRAM_ID>, 0.12999192 SOL reclaimed

$ solana balance
3.99873356 SOL
```

