# Rusting with style - Curso básico de linguagem Rust

<img src="../../rusting-crab-logo.png" alt="Descrição da imagem" style="height: 200px;">

[**Cleuton Sampaio**](https://linkedin.com/in/cleutonsampaio)

[**Veja no GitHub**](https://github.com/cleuton/rustingcrab)

[**Menu do curso**](../)

[**VÍDEO DESTA AULA**]()

# Match e outras coisas

A lição passada foi *punk* não? Bom, espero que essa seja mais suave para você... 

Você já ouviu falar em **pattern matching** (correspondência de padrões)? Em Rust, o `match` é uma construção de controle de fluxo poderosa que permite comparar um valor contra uma série de padrões e executar código com base no primeiro padrão que corresponder. É semelhante a um `switch` em outras linguagens, mas com muito mais flexibilidade e segurança.

Vamos a um exemplo: 

```rust
#[allow(dead_code)]
enum Dia {
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado,
    Domingo,
}

fn verificar_dia(dia: Dia) {
    match dia {
        Dia::Sabado | Dia::Domingo => {
            println!("É fim de semana!");
        },
        Dia::Segunda | Dia::Terca | Dia::Quarta | Dia::Quinta | Dia::Sexta => {
            println!("É um dia útil.");
        },
    }
}

fn main() {
    verificar_dia(Dia::Terca);
}
```

> `#[allow(dead_code)]` serve para desabilitar os avisos de código morto, ou seja, código que não foi usado. Esse exemplo vai encher o saco dizendo que os outros dias do `enum` não foram utilizados. Com isso, desabilitamos esse aviso. Só para esse exemplo! Não faça isso em produção. 

Vamos explicar o exemplo... Para começar, temos um `enum` que é exatamente como em qualquer outra linguagem. E fazemos um `match` em uma variável desse tipo, buscando por dias específicos. Se der `match` em **Sábado OU Domingo** ele avisa que é fim de semana, do contrário, avisa que é dia útil.

Note que estamos usando um só pipe ("|") em vez dos costumeiros dois pipes para OR ("||"). Por que? Porque em pattern matching é assim. Em condicionais usamos dois pipes. 


