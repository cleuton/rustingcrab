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
use crate::layer::Layer;
use crate::node::Node;
use crate::sinapse::Sinapse;
use rand::Rng;

pub struct Model {
    pub layers: Vec<Layer>,
    pub loss_value: f64,
    pub rng: rand::rngs::ThreadRng,
    pub first_layer: usize,
    pub last_layer: usize,
}

impl Model {
    pub fn new() -> Self {
        Model {
            layers: Vec::new(),
            loss_value: 0.0,
            rng: rand::thread_rng(),
            first_layer: 0,
            last_layer: 0,
        }
    }

    pub fn forward_pass(&mut self, input: Vec<f64>) -> Vec<f64> {
        let mut output_values = vec![0.0; self.layers[self.last_layer].nodes.len()];
        for (i, layer) in self.layers.iter_mut().enumerate() {
            if i == self.first_layer {
                for (j, node) in layer.nodes.iter_mut().enumerate() {
                    node.input = input[j];
                    node.value = input[j];
                }
            } else {
                for node in layer.nodes.iter_mut() {
                    let mut final_value = 0.0;
                    let previous = &self.layers[i - 1];
                    for nprev in &previous.nodes {
                        let sinapse = self.get_sinapse(nprev, node);
                        final_value += nprev.value * sinapse.weight;
                    }
                    let sinapse = self.get_sinapse(&previous.bias, node);
                    final_value += sinapse.weight;
                    node.input = final_value;
                    node.value = layer.activation.exec(node.input);
                }
            }
        }
        for (x, node) in self.layers[self.last_layer].nodes.iter().enumerate() {
            output_values[x] = node.value;
        }
        output_values
    }

    fn get_sinapse(&self, origem: &Node, destino: &Node) -> &Sinapse {
        origem.sinapses.iter().find(|s| s.final_node == destino as *const _ as usize).unwrap()
    }

    pub fn back_propagation(&mut self, target: Vec<f64>, learning_rate: f64) {
        let indice_ultima = self.last_layer;
        let ultima = &self.layers[indice_ultima];
        let qtd_saida = ultima.nodes.len();
        let mut output_errors = vec![0.0; qtd_saida];
        let mut outputs = vec![0.0; qtd_saida];
        for (x, node) in ultima.nodes.iter().enumerate() {
            output_errors[x] = node.value - target[x];
            outputs[x] = node.value;
        }
        for l in (0..indice_ultima).rev() {
            let layer = &mut self.layers[l];
            let proxima = &self.layers[l + 1];
            for node in &mut layer.nodes {
                if l == indice_ultima - 1 {
                    for sinapse in &mut node.sinapses {
                        let erro = output_errors[sinapse.final_node];
                        sinapse.gradient = erro * proxima.activation.calcular_derivada(proxima.nodes[sinapse.final_node].value) * node.value;
                    }
                } else {
                    for sinapse in &mut node.sinapses {
                        let mut valor_final = 0.0;
                        for s2 in &proxima.nodes[sinapse.final_node].sinapses {
                            let deltaz = output_errors[s2.final_node] * outputs[s2.final_node] * (1.0 - outputs[s2.final_node]);
                            valor_final += deltaz * s2.weight;
                        }
                        sinapse.gradient = valor_final * proxima.activation.calcular_derivada(proxima.nodes[sinapse.final_node].value) * node.value;
                    }
                }
            }
            if l == indice_ultima - 1 {
                for sinapse in &mut layer.bias.sinapses {
                    let erro = sinapse.final_node as f64 - target[sinapse.final_node];
                    sinapse.gradient = erro * layer.activation.calcular_derivada(layer.nodes[sinapse.final_node].value);
                }
            } else {
                for sinapse in &mut layer.bias.sinapses {
                    let mut valor_final = 0.0;
                    for s2 in &proxima.nodes[sinapse.final_node].sinapses {
                        let deltaz = output_errors[s2.final_node] * outputs[s2.final_node] * (1.0 - outputs[s2.final_node]);
                        valor_final += deltaz * s2.weight;
                    }
                    sinapse.gradient = valor_final * proxima.activation.calcular_derivada(proxima.nodes[sinapse.final_node].value);
                }
            }
        }
        for layer in &mut self.layers {
            for node in &mut layer.nodes {
                for sinapse in &mut node.sinapses {
                    sinapse.weight -= learning_rate * sinapse.gradient;
                }
            }
        }
    }

    pub fn fit(&mut self, dataset: Vec<Vec<f64>>, train_count: usize, epochs: usize, learning_rate: f64) {
        for epoch in 0..epochs {
            let mut mse = 0.0;
            for n in 0..train_count {
                let outputs = self.forward_pass(dataset[n].clone());
                for (z, output) in outputs.iter().enumerate() {
                    mse += (dataset[n][4 + z] - output).powi(2);
                }
                self.back_propagation(self.get_targets(&dataset[n]), learning_rate);
            }
            mse /= train_count as f64;
            println!("Epoch: {} MSE: {}", epoch, mse);
        }
    }

    fn get_targets(&self, ds: &Vec<f64>) -> Vec<f64> {
        vec![ds[4], ds[5], ds[6]]
    }

    pub fn calc_squared_errors(&self, target: Vec<f64>, estimated: Vec<f64>) -> f64 {
        target.iter().zip(estimated.iter()).map(|(t, e)| (t - e).powi(2)).sum()
    }

    pub fn get_random(&mut self) -> f64 {
        self.rng.gen_range(-1.0..1.0)
    }
}
