#[derive(Debug)]
pub struct Layer {
    pub number: usize, // Índice da layer no Model
    pub nodes: Vec<usize>,   // Índices dos Nodes que compõem a layer
    pub bias: uint,          // Índice do Node de bias
    pub activation: Box<dyn Activation>, // Função de ativação da layer (trait object)
}

impl Layer {
    pub fn new(numNodes: usize, 
                activation: Box<dyn Activation>,
                model: &mut model) -> Self {
        let number = model.layers.len();
        let mut layer = Layer {
            number: number,
            nodes: Vec::new(),
            bias: 0,
            activation: activation,
        }
        
        // adding a bias node:
        let bias = Node {
            layer_number: number,
            node_number: model.nodes.len(),
            sinapses: Vec::new(),
            input: 1.0,
            value: 1.0,
        };
        model.nodes.push(bias);
        layer.nodes.push(bias.node_number);
        layer.bias = bias.node_number;

        // Adicionando os nodes da layer
        for i in 0..numNodes {
            let node = Node {
                layer_number: number,
                node_number: model.nodes.len(),
                sinapses: Vec::new(),
                input: 0.0,
                value: 0.0,
            };
            model.nodes.push(node);
            layer.nodes.push(node.node_number);
        }

        // Sinapses da camada anterior para esta
        if layer.number > 0 {
            let prev_layer = &model.layers[layer.number - 1];  
            for i in 0..prev_layer.nodes.len() {
                let mut nprev = &model.nodes[prev_layer.nodes[i]];
                for j in 0..layer.nodes.len() {
                    let mut n = &model.nodes[layer.nodes[j]];
                    let sinapse = Sinapse::new(nprev.node_number, n.node_number, model.get_random());
                    model.sinapses.push(sinapse);
                    n.sinapses.push(model.sinapses.len() - 1);
                    nprev.sinapses.push(model.sinapses.len() - 1);
                }
            }
        } else {
            model.first_layer = layer.number;
        }
        model.last_layer = layer.number;
    }
}


pub PartialEq for Layer {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}

pub fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Layer {} with {} nodes", self.number, self.nodes.len())
    }
}
