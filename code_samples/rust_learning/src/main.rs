
fn main() {
    let mut model = Model::new(seed: 42);

    // Add layers: 
    model.layers.push(Layer::new(4, None, &mut model));
    model.layers.push(Layer::new(8, Some(Activation::Sigmoid{}), &mut model));
    model.layers.push(Layer::new(3, Some(Activation::Sigmoid{}), &mut model));

    for layer_idx in 0..model.layers.len() {
        let layer = &model.layers[layer_idx];
        println!("Layer: {}", layer.number);
        for node_idx in 0..layer.nodes.len() {
            let node = &model.nodes[layer.nodes[node_idx]];
            println!("Node: {} Value: {}", node.node_number, node.value);
        }
    }

    // Load dataset:
    let irisElementos = 150 as uint;
    let categorias = 3 as uint;
    let variaveis = 4 as uint;
    
    let iris = load_iris(irisElementos, categorias, variaveis); 

    // Train model:
    let epochs = 1000 as usize;
    let train_count = 120 as usize;
    let learning_rate = 0.001 as f64;

    model.fit(iris, train_count, epochs, learning_rate);

    // Test model:  
    let erros = 0 as uint;
    let contagem = 0 as uint;
    for n in 120..irisElementos {
        let mut testes = Vec::new();
        for i in 0..7 {
            testes.push(iris[n][i]);
        }
        let saidas = model.forward_pass(testes);
        println!("Entrada: {}", testes);
        println!("Calculado: {}", saidas);
        contagem +=1;
        let erro = false;
        for i in 0..3 {
            if (saidas[i].round() as u32) != testes[i + 4] as u32 {
                erro = true;
                break;
            }
        }
        if erro {
            erros += 1;
        }
    }



}