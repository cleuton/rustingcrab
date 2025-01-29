use crate::sinapse::Sinapse;
use crate::layer::Layer;
use crate::node::Node;
use crate::activation::Activation;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::RngCore;

pub struct Model {
    pub layers: Vec<Layer>, // Layers do modelo
    pub nodes: Vec<Node>,   // Nodes do modelo
    pub sinapses: Vec<Sinapse>, // Sinapses do modelo
    pub loss_value: f64,      // Valor da função de perda
    pub random: Box<StdRng>, // Gerador de números aleatórios
}

impl Model {

    pub fn add_layer(&mut self, num_nodes: usize, activation: Option<Box<dyn Activation>>, ultima: bool) {

        // Cria a camada e atualiza os nós/conexões internamente

        let mut layer = Layer::new(activation);
        layer.number = self.layers.len();
        println!("add Layer number: {}", layer.number);
        // Adiciona os nós da camada ao modelo:

        // bias node (exceto na primeira camada):
        if layer.number > 0 && !ultima {
            let bias = Node {
                layer_number: layer.number,
                node_number: self.nodes.len(),
                sinapses: Vec::new(),
                input: 0.0,
                value: 0.0,
            };
            println!("Bias node: {}", &bias.node_number);
            layer.bias = bias.node_number;
            self.nodes.push(bias);
        }

        // Outros nodes:
        for _node_ix in 0..num_nodes {
            let node = Node {
                layer_number: layer.number,
                node_number: self.nodes.len(),
                sinapses: Vec::new(),
                input: 0.0,
                value: 0.0,
            };
            println!("Node: {}", &node.node_number);
            layer.nodes.push(node.node_number);
            self.nodes.push(node);
        }

        // Sinapses da camada anterior para esta
        if layer.number > 0 {
            let prev_layer_ix = layer.number - 1;  
            for prev_nodes_ix in 0..self.layers[prev_layer_ix].nodes.len() {
                let prev_node_model_ix = self.layers[prev_layer_ix].nodes[prev_nodes_ix];
                for node_ix in 0..layer.nodes.len() {
                    let node_model_ix = layer.nodes[node_ix];
                    let sinapse = Sinapse::new(prev_node_model_ix, node_model_ix, self.get_random());
                    self.nodes[prev_node_model_ix].sinapses.push(self.sinapses.len());
                    self.sinapses.push(sinapse);
                }
            }
        }

        // Armazena a Layer (para evitar "move" antecipado):
        self.layers.push(layer);
    }



    pub fn forward_pass (&mut self, input: &Vec<f64>) -> Vec<f64> {
        let mut output_values = Vec::new();
        for layer_ix in 0..self.layers.len() {
            if 0 == layer_ix { 
                // First layer:
                let mut input_ix: usize = 0 as usize;
                // Bias node is not included in layer's nodes:
                for node_ix in 0..self.layers[layer_ix].nodes.len() {
                    let node = &mut self.nodes[self.layers[layer_ix].nodes[node_ix]];
                    node.input = input[input_ix];
                    node.value = input[input_ix];
                    input_ix += 1;
                }
            } else {
                // All other layers:
                for node_ix in 0..self.layers[layer_ix].nodes.len()-1 { 
                    let mut final_value: f64 = 0.0 as f64;
                    let p_layer_ix = layer_ix - 1; // avoid borrow checker
                    for p_node_ix in 0..self.layers[p_layer_ix].nodes.len() {
                        let previous_node_value = self.nodes[p_node_ix].value;
                        if p_node_ix == self.layers[p_layer_ix].bias {
                            // It is the previous' layer bias node:
                            final_value += self.get_sinapse_weight(p_node_ix, node_ix);
                        } else {
                            let sinapse_weight = self.get_sinapse_weight(p_node_ix, node_ix);
                            final_value += previous_node_value * sinapse_weight;
                            self.nodes[node_ix].input = final_value;
                            match &self.layers[layer_ix].activation {
                                Some(activation) => {
                                    self.nodes[node_ix].value = activation.exec(final_value);
                                }
                                None => {}
                            }
                        }
                    }
                }
            }
        }
        // Getting the output layer values:
        let last_layer = self.layers.len() - 1;
        println!("Last layer Nodes len: {}", self.layers[last_layer].nodes.len());
        for i in 0..self.layers[last_layer].nodes.len() {
            output_values.push(self.nodes[self.layers[last_layer].nodes[i]].value);
        }
        output_values
    }

    pub fn get_random(&mut self) -> f64 {
        self.random.next_u64() as f64 / u64::MAX as f64
    }

    fn get_sinapse_weight(&self, from: usize, to: usize) -> f64 {
        let mut sinapse_ix = 0;
        for i in 0..self.sinapses.len() {
            let sinapse = &self.sinapses[i];
            if sinapse.source_node == from && sinapse.dest_node == to {
                sinapse_ix = i;
                break;
            }
        }
        self.sinapses[sinapse_ix].weight
    }

    pub fn back_propagation(&mut self, target: Vec<f64>, learing_rate: f64) {
        let indice_ultima = self.layers.len() - 1;
        let qtd_saida = self.layers[indice_ultima].nodes.len();
        let mut output_errors = vec![0.0 as f64; qtd_saida];
        let mut outputs = vec![0.0 as f64; qtd_saida];
        for i in 0..qtd_saida {
            let node = &self.nodes[self.layers[indice_ultima].nodes[i]];
            outputs[i] = node.value;
            output_errors[i] = target[i] - node.value;
        }
        // Begin with penultimate layer:
        for layer_ix in (0..indice_ultima).rev() {
            println!("Layer_ix 151: {}", layer_ix);
            for node_ix in 0..self.layers[layer_ix].nodes.len() {
                // The penultimate layer is the first one to be processed:
                if layer_ix == indice_ultima - 1 {
                    println!("Node 155: {:?}", self.layers[layer_ix].nodes[node_ix]);
                    let current_node_ix = self.layers[layer_ix].nodes[node_ix];
                    println! ("Current_node 156: {}", current_node_ix);
                    for sinapse_ix in 0..self.nodes[current_node_ix].sinapses.len() {
                        let current_sinapse_ix = self.nodes[current_node_ix].sinapses[sinapse_ix];
                        println!("Sinapse_ix 156: {}", self.sinapses[current_sinapse_ix]);
                        let erro = output_errors[sinapse_ix];
                        let sinapse_final_node_value = self.nodes[self.sinapses[current_sinapse_ix].dest_node].value;
                        let activation = self.layers[layer_ix].activation.as_ref().expect("Activation function not found");
                        let new_gradient = erro * 
                            activation.calculate_derivative(sinapse_final_node_value) *
                            self.nodes[current_node_ix].value;
                        self.sinapses[current_sinapse_ix].gradient = new_gradient;  
                    }  
                } else {
                    for sinapse_ix in 0..self.nodes[node_ix].sinapses.len() {
                        let mut valor_final = 0.0 as f64;
                        let limite = self.nodes[self.sinapses[sinapse_ix].dest_node].sinapses.len();
                        for sinapse_final_ix in 0..limite {
                            let final_node_ix = self.sinapses[sinapse_final_ix].dest_node;
                            let deltaz = output_errors[final_node_ix] *
                                         outputs[final_node_ix] *
                                         (1 as f64 - outputs[final_node_ix]);
                            valor_final += deltaz * self.sinapses[sinapse_final_ix].weight;
                        }
                        let sinapse_final_node_value = self.nodes[self.sinapses[sinapse_ix].dest_node].value;
                        let activation = self.layers[layer_ix].activation.as_ref().expect("Activation function not found");
                        let new_gradient = valor_final * 
                                            activation.calculate_derivative(sinapse_final_node_value) * 
                                            self.nodes[node_ix].value;
                        self.sinapses[sinapse_ix].gradient = new_gradient;
                    }                      
                }  
            }
        }
        // Update the weights:
        for layer_ix in 0..(self.layers.len()-1) {
            let num_nodes = self.layers[layer_ix].nodes.len();
            for node_ix in 0..num_nodes {
                let num_sinapses = self.nodes[node_ix].sinapses.len();  
                for sinapse_ix in 0..num_sinapses {
                    let new_weight = self.sinapses[sinapse_ix].weight - 
                                     self.sinapses[sinapse_ix].gradient * learing_rate;
                    self.sinapses[sinapse_ix].weight = new_weight;
                }
            }
        }
    }

    pub fn fit(&mut self, dataset: &[Vec<f64>], train_count: usize, epochs: usize, learning_rate: f64) {
        let mut mse = 0.0 as f64;
        for _ in 0..epochs {
            mse = 0.0;
            for i in 0..train_count {
                // Extrair features (4 primeiros elementos)
                let input_features = &dataset[i][0..4].to_vec();
                
                // Extrair targets (3 últimos elementos)
                let targets = &dataset[i][4..7].to_vec();
                println!("Targets: {:?}", targets);
                // Forward pass apenas com as features
                let outputs = self.forward_pass(&input_features);
                println!("Outputs: {:?}", outputs);
                // Calcular MSE com os targets
                for j in 0..outputs.len() {
                    mse += (targets[j] - outputs[j]).powi(2);
                }
                
                // Backpropagation com os targets
                self.back_propagation(targets.clone(), learning_rate);
            }
            mse /= train_count as f64;
        }
        self.loss_value = mse;
    }

    pub fn new(seed: Option<usize>) -> Self {
        let rng = match seed {
            Some(seed) => SeedableRng::seed_from_u64(seed as u64),
            None => SeedableRng::from_entropy(),
        };
        Model {
            layers: Vec::new(),
            nodes: Vec::new(),
            sinapses: Vec::new(),
            loss_value: 0.0,
            random: Box::new(rng),
        }
    }

    pub fn calc_squared_errors(&self, target: &[f64], estimated: &[f64]) -> f64 {
        let mut retorno = 0.0;
        for y in 0..target.len() {
            retorno += (target[y] - estimated[y]).powi(2);
        }
        retorno
    }
}