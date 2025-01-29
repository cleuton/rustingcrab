use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::seq::SliceRandom;
use rust_learning::{Activation, Sigmoid, Model};



fn main() {
    let mut model = Model::new(Some(42));

    // Add layers: 
    model.add_layer(4, None, false);
    model.add_layer(8, Some(Box::new(Sigmoid{}) as Box<dyn Activation>), false);
    model.add_layer(3, Some(Box::new(Sigmoid{}) as Box<dyn Activation>), true);

    for layer_idx in 0..model.layers.len() {
        let layer = &model.layers[layer_idx];
        println!("Layer: {}", layer.number);
        println!("Nodes: {:?}", layer.nodes);
        for node_idx in 0..layer.nodes.len() {
            let node = &model.nodes[layer.nodes[node_idx]];
            println!("Node: {} Value: {}", node.node_number, node.value);
            for sinapse in &node.sinapses {
                println!("Sinapse: {}", model.sinapses[*sinapse]);
            }
        }
    }

    // Load dataset:
    let iris_elementos = 150 as usize;
    let categorias = 3 as usize;
    let variaveis = 4 as usize;
    
    let iris = load_iris(iris_elementos, categorias, variaveis); 

    // Train model:
    let epochs = 1000 as usize;
    let train_count = 120 as usize;
    let learning_rate = 0.001 as f64;

    model.fit(&iris, train_count, epochs, learning_rate);

    // Test model:  
    let mut erros = 0 as usize;
    let mut contagem = 0 as usize;
    for n in 120..iris_elementos {
        let mut testes = Vec::new();
        for i in 0..7 {
            testes.push(iris[n][i]);
        }
        let saidas = model.forward_pass(&testes);
        println!("Entrada: {:?}", &testes);
        println!("Calculado: {:?}", &saidas);
        contagem +=1;
        let mut erro = false;
        for i in 0..3 {
            if (saidas[i].round() as usize) != testes[i + 4] as usize {
                erro = true;
                break;
            }
        }
        if erro {
            erros += 1;
        }
        println!(
            "Testes: {} erros: {} acurácia: {:.2}%",
            contagem,
            erros,
            100.0 - (erros as f64 / contagem as f64) * 100.0
        );
    }
}

fn load_iris(elementos: usize, categorias: usize, variaveis: usize) -> Vec<Vec<f64>> {

    let mut dataset = vec![vec![0.0; variaveis + categorias]; elementos];
    let file = File::open("iris.data").expect("Cannot open iris.data file");
    let reader = BufReader::new(file);

    let mut linhas: Vec<String> = reader
        .lines()
        .filter_map(|line| line.ok())
        .take(elementos)
        .collect();

    let mut rng = rand::thread_rng();
    linhas.shuffle(&mut rng);

    for (reg, linha) in linhas.iter().enumerate() {
        let vetor1: Vec<&str> = linha.split(',').collect();
        for v in 0..variaveis {
            dataset[reg][v] = vetor1[v].parse().unwrap();
        }
        match vetor1[4] {
            "Iris-setosa" => {
                dataset[reg][variaveis] = 1.0;
                dataset[reg][variaveis + 1] = 0.0;
                dataset[reg][variaveis + 2] = 0.0;
            }
            "Iris-versicolor" => {
                dataset[reg][variaveis] = 0.0;
                dataset[reg][variaveis + 1] = 1.0;
                dataset[reg][variaveis + 2] = 0.0;
            }
            "Iris-virginica" => {
                dataset[reg][variaveis] = 0.0;
                dataset[reg][variaveis + 1] = 0.0;
                dataset[reg][variaveis + 2] = 1.0;
            }
            _ => panic!("Unknown iris class"),
        }
    }

    dataset
}