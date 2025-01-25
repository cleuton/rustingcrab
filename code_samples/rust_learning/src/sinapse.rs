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

use crate::node::Node;

#[derive(Debug)]
pub struct Sinapse {
    pub final_node: Node,
    pub weight: f64,
    pub gradient: f64,
}

impl std::fmt::Display for Sinapse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "\n\t[Sinapse. Final node number: {}/{} , weight: {}]",
            self.final_node.layer_number, self.final_node.node_number, self.weight
        )
    }
}

impl PartialEq for Sinapse {
    fn eq(&self, other: &Self) -> bool {
        self.final_node.layer_number == other.final_node.layer_number
            && self.final_node.node_number == other.final_node.node_number
    }
}