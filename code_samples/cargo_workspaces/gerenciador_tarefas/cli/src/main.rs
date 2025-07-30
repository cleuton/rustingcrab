use core::{GerenciadorDeTarefas, Tarefa};

fn main() {
    let mut gerenciador = GerenciadorDeTarefas::novo();

    let tarefa1 = Tarefa {
        id: 1,
        titulo: "Estudar Rust".to_string(),
        concluida: false,
    };

    gerenciador.adicionar_tarefa(tarefa1);

    println!("Tarefas cadastradas:");
    for tarefa in gerenciador.listar_tarefas() {
        println!("- [{}] {}", if tarefa.concluida { "x" } else { " " }, tarefa.titulo);
    }
}