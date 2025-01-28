use std::fmt;
use crate::activation::Activation;

#[derive(Debug)]
pub struct Layer {
    pub number: usize, // Índice da layer no Model
    pub nodes: Vec<usize>,   // Índices dos Nodes que compõem a layer
    pub bias: usize,          // Índice do Node de bias
    pub activation: Option<Box<dyn Activation>>, // Função de ativação da layer (trait object)
}

impl Layer {
    pub fn new(activation: Option<Box<dyn Activation>>) -> Self {
        let layer = Layer {
            number: 0 as usize,
            nodes: Vec::new(),
            bias: 0,
            activation: activation,
        };
        layer
    }
}


impl PartialEq for Layer {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Layer {} with {} nodes", self.number, self.nodes.len())
    }
}
