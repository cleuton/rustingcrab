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