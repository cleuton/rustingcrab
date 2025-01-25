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

/* 
package com.neuraljava.samples.mlpgen.api;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class IrisClassifier {
	
	public static void main (String [] args) throws IOException {
		Logger logger = LoggerFactory.getLogger(IrisClassifier.class);
		Model model = new Model(42);
		model.layers.add(new Layer(4,null,model)); // Input layer não tem activation
		model.layers.add(new Layer(8,new Sigmoid(),model));
		model.layers.add(new Layer(3,new Sigmoid(),model));
		
		for (Layer l : model.layers) {
			System.out.println(l);
		}
		
		int irisElementos = 150;
		int categorias = 3;
		int variaveis = 4;
		int epochs = 1000;
		double learningRate = 0.01;
		
		double [][] iris = loadIris(irisElementos, categorias, variaveis);		
		
		model.fit(iris, 120, epochs,learningRate);
		
		int erros = 0;
		int contagem = 0;
		for (int n=120;n<irisElementos;n++) {
			double [] testes = new double[7];
			for (int i=0;i<7;i++) {
				testes[i] = iris[n][i];
			}
			double [] saidas = model.forwardPass(testes);
			System.out.println("Entrada: " + Arrays.toString(testes));
			System.out.println("Calculado: " + Arrays.toString(saidas));
			contagem++;
			boolean erro = false;
			for (int i=0;i<3;i++) {
				erro = Math.round(saidas[i]) == testes[i+4] ? false : true;
			}
			if (erro) {
				erros++;
			}
		}
		
		System.out.println("Testes: " + contagem + " erros: " + erros + " acurácia: " + (100.0 - ((double)erros/(double)contagem)*100) + "%");
		
	}

	private static double [][] loadIris(int elementos, int categorias, int variaveis) throws IOException {
		double [][] dataset = new double [elementos][variaveis + categorias];
		BufferedReader br = new BufferedReader(new InputStreamReader(
				IrisClassifier.class.getResourceAsStream("/iris.data")));
		String linha = "";
		String [] linhas = new String [elementos];
		for (int x=0;x<elementos;x++) {
			linha = br.readLine();
			if (linha.length()>0) {
				linhas[x] = linha;
			}
		}
		List<String> registros = Arrays.asList(linhas);
		Collections.shuffle(registros);
		int reg = 0;
		for (String linha2 : registros) {
			String [] vetor1 = linha2.split(",");
			for (int v=0; v<variaveis;v++) {
				dataset[reg][v] = Double.parseDouble(vetor1[v]);
			}
			// One hot encoding: 1 0 0 - "Iris-setosa" 0 1 0 - "Iris-versicolor" 0 0 1 - "Iris-virginica"
			if (vetor1[4].equals("Iris-setosa")) {
				dataset[reg][4] = 1.0;
				dataset[reg][5] = 0.0;
				dataset[reg][6] = 0.0;
			}
			else if (vetor1[4].equals("Iris-versicolor")) {
				dataset[reg][4] = 0.0;
				dataset[reg][5] = 1.0;
				dataset[reg][6] = 0.0;				
			}
			else {
				dataset[reg][4] = 0.0;
				dataset[reg][5] = 0.0;
				dataset[reg][6] = 1.0;				
			}
			reg++;
		}
		return dataset;
	}
	
}
*/

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::VecDeque;
use std::collections::LinkedList;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::collections::VecDeque;


fn main() {
    let mut model = Model::new(42);
    model.layers.push(Layer::new(4,None,&model));
    model.layers.push(Layer::new(8,Some(Box::new(Sigmoid{})),&model));
    model.layers.push(Layer::new(3,Some(Box::new(Sigmoid{})),&model));
    
    for l in &model.layers {
        println!("{}",l);
    }
    
    let iris_elementos = 150;
    let categorias = 3;
    let variaveis = 4;
    let epochs = 1000;
    let learning_rate = 0.01;
    
    let iris = load_iris(iris_elementos, categorias, variaveis);        
    
    model.fit(&iris, 120, epochs,learning_rate);
    
    let mut erros = 0;
    let mut contagem = 0;
    for n in 120..iris_elementos {
        let mut testes = vec![0.0;7];
        for i in 0..7 {
            testes[i] = iris[n][i];
        }
        let saidas = model.forward_pass(&testes);
        println!("Entrada: {:?}",testes);
        println!("Calculado: {:?}",saidas);
        contagem += 1;
        let mut erro = false;
        for i in 0..3 {
            erro = (saidas[i].round() as i32) == testes[i+4] as i32;
        }
        if erro {
            erros += 1;
        }
    }
    
    println!("Testes: {} erros: {} acurácia: {}%", contagem, erros, 100.0 - ((erros as f64)/(contagem as f64))*100.0);
    
}


