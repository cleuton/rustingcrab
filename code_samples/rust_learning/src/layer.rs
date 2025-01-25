/*

Copyright 2018 Cleuton Sampaio

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License. 

Este trabalho é para demonstração de redes neurais e não tem objetivo 
de desempenho ou precisão. O Autor não se responsabiliza pelo seu uso e
não fornecerá suporte. 
*/

use crate::activation::Activation;
use crate::model::Model;
use crate::node::Node;
use crate::sinapse::Sinapse;
use std::fmt;

#[derive(Debug)]
pub struct Layer {
    pub activation: Box<dyn Activation>,
    pub nodes: Vec<Node>,
    pub number: i32,
    pub bias: Node,
}

impl Layer {
    pub fn new(num_nodes: i32, 
        activation: impl Activation + 'static,
        model: &mut Model) -> Layer {
        let mut nodes: Vec<Node> = Vec::new();
        let mut bias = Node::new();
        bias.sinapses = Vec::new();
        let number = model.layers.len() as i32 + 1;
        let mut layer = Layer {
            activation: Box::new(activation),
            nodes,
            number,
            bias,
        };
        for x in 0..num_nodes {
            let mut node = Node::new();
            node.sinapses = Vec::new();
            layer.nodes.push(node);
            node.layer_number = layer.number;
            node.node_number = x + 1;
        }
        // Criamos as sinapses da camada anterior, conectando esta camada à ela.
        if model.layers.len() > 0 {
            // A input layer não tem camada anterior
            let previous = &model.layers[model.layers.len() - 1]; // Pega a última inserida
            for nprev in &previous.nodes {
                for natu in &layer.nodes {
                    let mut sinapse = Sinapse::new();
                    sinapse.final_node = natu.clone();
                    sinapse.weight = model.get_random();
                    nprev.sinapses.push(sinapse);
                }
            }
            // Bias da camada anterior (um pouco de repetição, mas dá para entender bem)
            for natu in &layer.nodes {
                let mut sinapse = Sinapse::new();
                sinapse.final_node = natu.clone();
                sinapse.weight = model.get_random();
                previous.bias.sinapses.push(sinapse);
            }
        } else {
            model.first_layer = layer.clone();
        }
        model.last_layer = layer.clone();
        layer
    }
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\n[Layer. Number : {}\nBias: {}\nnodes:\n{}]",
            self.number, self.bias, self.nodes.len()
        )
    }
}

impl PartialEq for Layer {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}

impl Eq for Layer {}

