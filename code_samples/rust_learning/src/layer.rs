/*

Copyright 2024 Cleuton Sampaio

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

use std::vec::Vec;
use crate::activation::Activation;
use crate::node::Node;
use crate::sinapse::Sinapse;
use crate::model::Model;

#[derive(Debug, PartialEq)]
pub struct Layer {
    pub activation: Activation,
    pub nodes: Vec<Node>,
    pub number: usize,
    pub bias: Node,
}

impl Layer {
    pub fn new(num_nodes: usize, activation: Activation, model: &mut Model) -> Self {
        let number = model.layers.len() + 1;
        let mut nodes = Vec::new();
        let mut bias = Node::new();
        bias.sinapses = Vec::new();
        bias.layer_number = number;

        for x in 0..num_nodes {
            let mut node = Node::new();
            node.sinapses = Vec::new();
            node.layer_number = number;
            node.node_number = x + 1;
            nodes.push(node);
        }

        if !model.layers.is_empty() {
            let previous = model.layers.last().unwrap();
            for nprev in &previous.nodes {
                for natu in &nodes {
                    let mut sinapse = Sinapse::new();
                    sinapse.final_node = Some(Box::new(natu.clone()));
                    sinapse.weight = model.get_random();
                    nprev.sinapses.borrow_mut().push(sinapse);
                }
            }

            for natu in &nodes {
                let mut sinapse = Sinapse::new();
                sinapse.final_node = Some(Box::new(natu.clone()));
                sinapse.weight = model.get_random();
                previous.bias.sinapses.borrow_mut().push(sinapse);
            }
        } else {
            model.first_layer = Some(Box::new(Layer {
                activation: activation.clone(),
                nodes: nodes.clone(),
                number,
                bias: bias.clone(),
            }));
        }

        model.last_layer = Some(Box::new(Layer {
            activation: activation.clone(),
            nodes: nodes.clone(),
            number,
            bias: bias.clone(),
        }));

        Layer {
            activation,
            nodes,
            number,
            bias,
        }
    }
}

impl ToString for Layer {
    fn to_string(&self) -> String {
        format!(
            "\n[Layer. Number : {}\nBias: {:?}\nnodes:\n{:?}\n]",
            self.number, self.bias, self.nodes
        )
    }
}