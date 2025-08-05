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
            "Olá, {}! Este código foi criado em {}."
        }}
        "#,
        nome,
        now.format("%d/%m/%Y %H:%M:%S")
    );

    // Grava na saída
    fs::write(&caminho_saida, saudacao)
        .expect("Falhou ao gerar o código de saudação");

    // Avisa o Cargo para recompilar se build.rs mudar
    println!("cargo:rerun-if-changed=build.rs");
}