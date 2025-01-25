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

use crate::sinapse::Sinapse;
use crate::activation::Activation;

pub struct Node {
    pub layer_number: i32,
    pub node_number: i32,
    pub sinapses: Vec<Sinapse>,
    pub input: f64,  // net value before activation
    pub value: f64,  // output or current value
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        return other.input == self.input && other.layer_number == self.layer_number;
    }
}

impl Node {
    pub fn new(layer_number: i32, node_number: i32) -> Node {
        return Node {
            layer_number: layer_number,
            node_number: node_number,
            sinapses: Vec::new(),
            input: 0.0,
            value: 0.0
        };
    }
    pub fn calculate_value(&mut self, activation: &dyn Activation) {
        self.value = activation.exec(self.input);
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\n\t[Node. Layer: {}, node: {}, input: {}, value: {}, sinapses: {:?}]", 
               self.layer_number, self.node_number, self.input, self.value, self.sinapses)
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\n\t[Node. Layer: {}, node: {}, input: {}, value: {}, sinapses: {:?}]", 
               self.layer_number, self.node_number, self.input, self.value, self.sinapses)
    }
}

