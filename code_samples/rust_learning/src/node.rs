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

use std::fmt;
use std::cmp::PartialEq;
use crate::sinapse::Sinapse;

#[derive(Debug)]
pub struct Node {
     pub layer_number: i32,
     pub node_number: i32,
     pub sinapses: Vec<Sinapse>,
     pub input: f64,  // net value before activation
     pub value: f64,  // output or current value
}

impl PartialEq for Node {
     fn eq(&self, other: &Self) -> bool {
          self.input == other.input && self.layer_number == other.layer_number
     }
}

impl fmt::Display for Node {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
          write!(f, "\n\t[Node. Layer: {}, node: {}, input: {}, value: {}, sinapses: {:?}]",
                    self.layer_number, self.node_number, self.input, self.value, self.sinapses)
     }
}

