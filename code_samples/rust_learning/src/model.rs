use crate::activation::Activation;
use crate::sinapse::Sinapse;
use crate::layer::Layer;
use crate::node::Node;
use rand::RngCore;

pub struct Model {
    pub layers: Vec<Layer>, // Layers do modelo
    pub nodes: Vec<Node>,   // Nodes do modelo
    pub sinapses: Vec<Sinapse>, // Sinapses do modelo
    pub f64 loss_value;      // Valor da função de perda
    pub random: rng: Box<dyn RngCore>, // Gerador de números aleatórios
}

impl Model {
    fn get_random() -> f64 {
        self.random.next_u64() as f64 / u64::MAX as f64
    }
}