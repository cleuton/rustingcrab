use csv;
use ndarray::prelude::*;
use ndarray_rand::RandomExt;
use rand::distributions::Uniform;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

/// Função de ativação sigmoide.
fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}

/// Derivada da sigmoide em função da própria ativação.
/// Se a = sigma(z), então sigma'(z) = a * (1 - a).
fn derivada_sigmoide(a: f32) -> f32 {
    a * (1.0 - a)
}

/// Calcula o produto externo (produto_externo product) entre dois vetores.
/// Se `a` tem dimensão (n,) e `b` tem dimensão (m,),
/// o resultadoado será uma matriz de dimensão (n, m) onde resultado[[i, j]] = a[i] * b[j].
fn produto_externo(a: &Array1<f32>, b: &Array1<f32>) -> Array2<f32> {
    let n = a.len();
    let m = b.len();
    let mut resultado = Array2::<f32>::zeros((n, m));
    for i in 0..n {
        for j in 0..m {
            resultado[[i, j]] = a[i] * b[j];
        }
    }
    resultado
}

/// Representação da rede neural:
/// - pesos: vetor de matrizes onde cada matriz conecta uma camada à próxima.
/// - biases: vetor de vetores com os vieses de cada camada.
struct RedeNeural {
    pesos: Vec<Array2<f32>>,
    biases: Vec<Array1<f32>>,
}

impl RedeNeural {
    /// Cria uma nova rede dada a lista de tamanhos de camada.
    /// Por exemplo, tamanho_camadas = &[4, 8, 3] cria uma rede com 4 entradas, 8 neurônios na camada oculta e 3 saídas.
    fn new(tamanho_camadas: &[usize]) -> Self {
        let mut pesos = Vec::new();
        let mut biases = Vec::new();
        let dist = Uniform::new(-1.0, 1.0);
        for janela in tamanho_camadas.windows(2) {
            let (n_in, n_out) = (janela[0], janela[1]);
            pesos.push(Array2::random((n_out, n_in), dist));
            biases.push(Array1::random(n_out, dist));
        }
        RedeNeural { pesos, biases }
    }

    /// Propagação para frente simples: dado um vetor de entrada, retorna a ativação da camada de saída.
    fn propagacao_frente(&self, entrada: &Array1<f32>) -> Array1<f32> {
        let mut ativacao = entrada.clone();
        for (w, b) in self.pesos.iter().zip(self.biases.iter()) {
            // z = W * a + b
            let z = w.dot(&ativacao) + b;
            // a = sigma(z)
            ativacao = z.mapv(sigmoid);
        }
        ativacao
    }

    /// Propagação para frente que armazena as ativações e os valores z (pré-ativação) de cada camada.
    /// Retorna uma tupla: (vetor de ativações, vetor de z's).
    fn propagacao_frente_com_intermediarios(
        &self,
        entrada: &Array1<f32>,
    ) -> (Vec<Array1<f32>>, Vec<Array1<f32>>) {
        let mut ativacoes = Vec::new();
        let mut zs = Vec::new();
        let mut ativacao = entrada.clone();
        ativacoes.push(ativacao.clone());
        for (w, b) in self.pesos.iter().zip(self.biases.iter()) {
            let z = w.dot(&ativacao) + b;
            zs.push(z.clone());
            ativacao = z.mapv(sigmoid);
            ativacoes.push(ativacao.clone());
        }
        (ativacoes, zs)
    }

    /// Executa um passo de treinamento para cada exemplo dos dados de treinamento.
    /// Aqui usamos *Stochastic Gradient Descent* (SGD) sem mini-batches para simplicidade.
    fn treinar(
        &mut self,
        entradas: &Vec<Array1<f32>>,
        alvos: &Vec<Array1<f32>>,
        learning_rate: f32,
    ) {
        // Para cada exemplo, realizamos a propagação para frente, o backpropagation e atualizamos os pesos.
        for (entrada, alvo) in entradas.iter().zip(alvos.iter()) {
            // Propagação para frente armazenando ativações e valores z
            let (ativacoes, _zs) = self.propagacao_frente_com_intermediarios(entrada);
            let num_camadas = self.pesos.len();

            // Vetores para armazenar os gradientes (nabla) para vieses e pesos
            let mut nabla_b = Vec::with_capacity(num_camadas);
            let mut nabla_w = Vec::with_capacity(num_camadas);

            // --- Backpropagation ---

            // Para a camada de saída (última camada):
            // delta = (a_l - y) * sigma'(a_l)  (lembrando que a_l = sigma(z_L))
            let a_l = &ativacoes[num_camadas];
            let mut delta = a_l.mapv(|_| 0.0);
            for i in 0..a_l.len() {
                delta[i] = (a_l[i] - alvo[i]) * derivada_sigmoide(a_l[i]);
            }
            // Gradiente para os vieses da camada de saída:
            nabla_b.push(delta.clone());
            // Gradiente para os pesos da camada de saída: produto_externo product entre delta e a^(L-1)
            nabla_w.push(produto_externo(&delta, &ativacoes[num_camadas - 1]));

            // Backpropagate para as camadas ocultas
            let mut delta_corrente = delta;
            // Itera de l = num_camadas-1 até 1 (índices para pesos e biases: 0 .. num_camadas-1)
            for l in (1..num_camadas).rev() {
                let w_next = &self.pesos[l];
                let a_l = &ativacoes[l];
                let mut delta_proximo = w_next.t().dot(&delta_corrente);
                for i in 0..delta_proximo.len() {
                    delta_proximo[i] *= derivada_sigmoide(a_l[i]);
                }
                nabla_b.insert(0, delta_proximo.clone());
                nabla_w.insert(0, produto_externo(&delta_proximo, &ativacoes[l - 1]));
                delta_corrente = delta_proximo;
            }

            // --- Atualização dos parâmetros ---
            for i in 0..num_camadas {
                self.pesos[i] = &self.pesos[i] - &(learning_rate * &nabla_w[i]);
                self.biases[i] = &self.biases[i] - &(learning_rate * &nabla_b[i]);
            }
        }
    }

    /// Calcula o Mean Squared Error (MSE) sobre um conjunto de dados.
    fn mse(&self, entradas: &Vec<Array1<f32>>, alvos: &Vec<Array1<f32>>) -> f32 {
        let mut sum = 0.0;
        let n = entradas.len() as f32;
        for (entrada, alvo) in entradas.iter().zip(alvos.iter()) {
            let output = self.propagacao_frente(entrada);
            let diff = &output - alvo;
            sum += diff.mapv(|x| x * x).sum();
        }
        sum / n
    }
}

/// Converte a string de classe em um vetor one-hot encoding com 3 posições.
fn one_hot(label: &str) -> Array1<f32> {
    match label.trim() {
        "Iris-setosa" => array![1.0, 0.0, 0.0],
        "Iris-versicolor" => array![0.0, 1.0, 0.0],
        "Iris-virginica" => array![0.0, 0.0, 1.0],
        _ => panic!("Classe desconhecida: {}", label),
    }
}

/// Lê o dataset Iris a partir de um arquivo CSV e retorna um vetor de (entrada, alvo).
fn carrega_iris_dataset(path: &str) -> Result<Vec<(Array1<f32>, Array1<f32>)>, Box<dyn Error>> {
    let arq = File::open(path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(BufReader::new(arq));
    let mut dataset = Vec::new();

    for resultado in rdr.records() {
        let registro = resultado?;
        if registro.len() < 5 {
            continue; // pula linhas inválidas
        }
        // Parse dos 4 entradas:
        let entrada: Array1<f32> = array![
            registro[0].parse::<f32>()?,
            registro[1].parse::<f32>()?,
            registro[2].parse::<f32>()?,
            registro[3].parse::<f32>()?
        ];
        // Última coluna: label
        let alvo = one_hot(&registro[4]);
        dataset.push((entrada, alvo));
    }
    Ok(dataset)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Lê o dataset do arquivo "iris.data".
    let mut dataset = carrega_iris_dataset("iris.data")?;
    println!("Total de registros: {}", dataset.len());

    // Embaralha o dataset
    let mut rng = thread_rng();
    dataset.shuffle(&mut rng);

    // Separa em treino (80%) e teste (20%)
    let indice_separacao = (dataset.len() as f32 * 0.8).round() as usize;
    let (conjunto_treino, conjunto_teste) = dataset.split_at(indice_separacao);

    // Separa entradas e alvos para cada conjunto.
    let treinar_entradas: Vec<Array1<f32>> = conjunto_treino.iter().map(|(x, _)| x.clone()).collect();
    let treinar_alvos: Vec<Array1<f32>> = conjunto_treino.iter().map(|(_, y)| y.clone()).collect();

    let testar_entradas: Vec<Array1<f32>> = conjunto_teste.iter().map(|(x, _)| x.clone()).collect();
    let testar_alvos: Vec<Array1<f32>> = conjunto_teste.iter().map(|(_, y)| y.clone()).collect();

    println!(
        "Treino: {} registros, Teste: {} registros",
        treinar_entradas.len(),
        testar_entradas.len()
    );

    // Configuração da rede: 4 entradas, 8 neurônios na camada oculta e 3 saídas.
    let tamanho_camadas = vec![4, 8, 3];
    let mut rede = RedeNeural::new(&tamanho_camadas);

    let learning_rate = 0.05;
    let epochs = 500;

    // Loop de treinamento
    for epoch in 0..epochs {
        rede.treinar(&treinar_entradas, &treinar_alvos, learning_rate);
        if epoch % 50 == 0 {
            let treinar_error = rede.mse(&treinar_entradas, &treinar_alvos);
            let test_error = rede.mse(&testar_entradas, &testar_alvos);
            println!(
                "Epoch {}: MSE Treino = {:.6}, MSE Teste = {:.6}",
                epoch, treinar_error, test_error
            );
        }
    }

    // Exemplo de predição: escolhe um registro do conjunto de teste.
    let indice_exemplo = 0;
    let entrada_exemplo = &testar_entradas[indice_exemplo];
    let alvo_exemplo = &testar_alvos[indice_exemplo];
    let saida_exemplo = rede.propagacao_frente(entrada_exemplo);
    println!("\nExemplo de predição:");
    println!("Entrada: {:?}", entrada_exemplo);
    println!("Saída prevista: {:?}", saida_exemplo);
    println!("alvo: {:?}", alvo_exemplo);

    Ok(())
}
