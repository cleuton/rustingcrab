use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Tarefa {
    pub id: u32,
    pub titulo: String,
    pub concluida: bool,
}

pub struct GerenciadorDeTarefas {
    tarefas: Vec<Tarefa>,
}

impl GerenciadorDeTarefas {
    pub fn novo() -> Self {
        GerenciadorDeTarefas {
            tarefas: Vec::new(),
        }
    }

    pub fn adicionar_tarefa(&mut self, tarefa: Tarefa) {
        self.tarefas.push(tarefa);
    }

    pub fn listar_tarefas(&self) -> &Vec<Tarefa> {
        &self.tarefas
    }
}
