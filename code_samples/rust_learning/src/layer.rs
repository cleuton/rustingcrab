use std::fmt;
use crate::activation::Activation;
use crate::model::Model;
use crate::node::Node;
use crate::sinapse::Sinapse;

#[derive(Debug)]
pub struct Layer {
    pub number: usize, // Índice da layer no Model
    pub nodes: Vec<usize>,   // Índices dos Nodes que compõem a layer
    pub bias: usize,          // Índice do Node de bias
    pub activation: Option<Box<dyn Activation>>, // Função de ativação da layer (trait object)
}

impl Layer {
    pub fn new(num_nodes: usize, 
                activation: Option<Box<dyn Activation>>,
                model: &mut Model) -> Self {
        let number = model.layers.len();
        let mut layer = Layer {
            number: number,
            nodes: Vec::new(),
            bias: 0,
            activation: activation,
        };
        
        // adding a bias node:
        let bias = Node {
            layer_number: number,
            node_number: model.nodes.len(),
            sinapses: Vec::new(),
            input: 1.0,
            value: 1.0,
        };
        layer.nodes.push(bias.node_number);
        layer.bias = bias.node_number;
        model.nodes.push(bias);

        // Adicionando os nodes da layer
        for i in 0..num_nodes {
            let node = Node {
                layer_number: number,
                node_number: i,
                sinapses: Vec::new(),
                input: 0.0,
                value: 0.0,
            };
            layer.nodes.push(node.node_number);
            model.nodes.push(node);
        }

        // Sinapses da camada anterior para esta
        if layer.number > 0 {
            let prev_layer = &model.layers[layer.number - 1];  
            for i in 0..prev_layer.nodes.len() {
                //let nprev = &model.nodes[prev_layer.nodes[i]];
                let nprev_ix = i;
                for j in 0..layer.nodes.len() {
                    let sinapse = Sinapse::new(nprev_ix, j, model.get_random());
                    model.sinapses.push(sinapse);
                    model.nodes[j].sinapses.push(model.sinapses.len() - 1);
                    model.nodes[nprev_ix].sinapses.push(model.sinapses.len() - 1);
                }
            }
        }
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
