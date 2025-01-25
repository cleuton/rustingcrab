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
/*package com.neuraljava.samples.mlpgen.api;

public class Sinapse {
	public Node finalNode;
	public double weight;
	public double gradient;
	@Override
	public String toString() {
		String saida = "\n\t[Sinapse. Final node number: "
					 + finalNode.layerNumber + "/" + finalNode.nodeNumber
					 + ", weight: "
					 + this.weight
					 + "]";
		return saida;
	}
	@Override
	public boolean equals(Object obj) {
		Sinapse sinapse = (Sinapse) obj;
		Node outro = sinapse.finalNode;
		Node node = this.finalNode;
		return node.layerNumber == outro.layerNumber
				&& node.nodeNumber == outro.nodeNumber;
	}
	
}*/

use crate::node::Node;

pub struct Sinapse {
    pub final_node: Node,
    pub weight: f64,
    pub gradient: f64
}

impl Sinapse {
    pub fn new(final_node: Node, weight: f64) -> Sinapse {
        Sinapse {
            final_node,
            weight,
            gradient: 0.0
        }
    }
}

impl PartialEq for Sinapse {
    fn eq(&self, other: &Self) -> bool {
        self.final_node.layer_number == other.final_node.layer_number
            && self.final_node.node_number == other.final_node.node_number
    }
}

impl std::fmt::Debug for Sinapse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\n\t[Sinapse. Final node number: {}/{} weight: {}]", 
            self.final_node.layer_number, self.final_node.node_number, self.weight)
    }
}

