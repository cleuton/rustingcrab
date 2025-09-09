<img src="gossip.png" height=300>

# Demonstração do Padrão de Disseminação Gossip

Este projeto é uma implementação prática e didática do **padrão de disseminação Gossip** (também conhecido como *Epidemic Broadcast*). Ele simula uma rede descentralizada de nós que compartilham e sincronizam seu estado interno de forma eficiente e robusta, imitando como um boato se espalha em um grupo social.

## O que é o Padrão Gossip?

O padrão Gossip é um mecanismo de comunicação descentralizado usado em sistemas distribuídos para propagar informações de maneira confiável, mesmo na presença de falhas de rede ou nós. Em vez de um nó enviar uma atualização para todos os outros (o que é ineficiente e frágil), ele envia a informação para apenas um ou poucos nós selecionados aleatoriamente. Esses nós, por sua vez, repassam a informação adiante em rodadas subsequentes.

**Principais características e vantagens:**

*   **Descentralizado:** Não há um ponto único de controle ou falha.
*   **Robusto:** A rede continua funcionando mesmo que alguns nós falhem ou fiquem inacessíveis.
*   **Escalável:** O tráfego de rede cresce de forma manejável conforme o número de nós aumenta.
*   **Eventualmente Consistente:** Embora não haja garantia de sincronização instantânea, todos os nós ativos acabarão convergindo para o mesmo estado após um tempo suficiente.

Esse padrão é amplamente utilizado em bancos de dados distribuídos (como Cassandra e DynamoDB), sistemas de descoberta de serviços e redes peer-to-peer.

## Como a Implementação Funciona

Este código em Rust cria um nó de gossip que pode ser executado múltiplas vezes para formar uma rede. Cada nó possui:

1.  **Estado Interno (`NodeState`):** Um mapa de chave-valor onde cada entrada também possui um número de versão. Isso permite que os nós determinem qual valor é o mais recente durante a sincronização.
2.  **Lista de Peers (`peers`):** Um registro dos endereços dos outros nós na rede com os quais ele pode se comunicar.
3.  **Comunicação via UDP:** Usa o protocolo UDP para enviar e receber mensagens de forma leve e sem conexão.

O nó executa quatro tarefas principais de forma assíncrona:

*   **Escuta de Mensagens:** Fica constantemente escutando na porta designada por mensagens de gossip de outros nós.
*   **Disseminação Periódica (Fofoca):** A cada 2 segundos, escolhe aleatoriamente um peer da sua lista e envia a ele uma mensagem contendo seu estado completo e sua lista de peers.
*   **Processamento de Mensagens Recebidas:** Quando recebe uma mensagem, o nó:
    *   **Funde os Estados:** Compara as versões das chaves. Se o valor recebido tiver uma versão mais alta, ele atualiza seu estado local.
    *   **Atualiza a Lista de Peers:** Adiciona o endereço do remetente e todos os peers que o remetente conhece à sua própria lista, descobrindo assim novos nós na rede.
*   **Interface de Usuário:** Permite que o usuário insira comandos no terminal para atualizar o estado local (ex: `set minha_chave "meu_valor"`). Essas alterações são então propagadas para a rede via gossip.

## Instruções para Compilação e Execução

### Pré-requisitos
Certifique-se de ter o **Rust** e o **Cargo** instalados. Se ainda não os tiver, instale-os através do [rustup](https://www.rust-lang.org/tools/install).

### Passo a Passo para demonstrar

1.  **Execute a rede:**
    Abra **múltiplos terminais** para simular diferentes nós na rede.

    *   **Terminal 1 (Primeiro Nó - Semente):**
        ```bash
        cargo run -- --port 8000
        ```
        Este é o primeiro nó da rede. Como não há ninguém para se conectar, ele inicia sozinho.

    *   **Terminal 2 (Segundo Nó):**
        ```bash
        cargo run -- --port 8001 --peer 127.0.0.1:8000
        ```
        Este nó se conecta ao primeiro nó (`--peer 127.0.0.1:8000`).

    *   **Terminal 3 (Terceiro Nó):**
        ```bash
        cargo run -- --port 8002 --peer 127.0.0.1:8001
        ```
        Este nó pode se conectar a qualquer nó já existente na rede (neste caso, ao segundo nó).

2.  **Teste a Disseminação:**
    Em qualquer um dos terminais, digite um comando para atualizar o estado local:
    ```
    set my_key "hello world"
    ```
    Pressione `Enter`. Em poucos segundos, você verá logs nos outros terminais indicando que o estado foi atualizado, demonstrando como a informação se espalhou pela rede via gossip.

3.  **Monitore o Estado:**
    A cada 10 segundos, cada nó imprime automaticamente seu estado atual no terminal, permitindo que você visualize a convergência dos dados.


