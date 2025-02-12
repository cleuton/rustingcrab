/// Implementação rede convolucional para dataset MINST - (c) 2025 por Cleuton Sampaio

use mnist::{Mnist, MnistBuilder};
use ndarray::{Array, Array1, Array2, Array3, Array4};
use rand::distributions::Uniform;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

/// Função de ativação sigmoide e sua derivada (calculada a partir da saída)
fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}

/// Derivada da sigmoide a partir da saída: se a = sigmoid(z) então a * (1 - a)
fn sigmoid_derivada_da_saida(a: f32) -> f32 {
    a * (1.0 - a)
}

/// Função auxiliar para calcular o produto externo entre dois vetores.
fn produto_externo(a: &Array1<f32>, b: &Array1<f32>) -> Array2<f32> {
    let n = a.len();
    let m = b.len();
    let mut result = Array2::<f32>::zeros((n, m));
    for i in 0..n {
        for j in 0..m {
            result[[i, j]] = a[i] * b[j];
        }
    }
    result
}

/// ---------------------------------------------------------------------
/// Camada Convolucional
/// ---------------------------------------------------------------------
struct CamadaConvolucional {
    num_filters: usize,
    filter_height: usize,
    filter_width: usize,
    in_channels: usize,
    // Filtros com formato: (num_filters, in_channels, filter_height, filter_width)
    filters: Array4<f32>,
    // Bias para cada filtro
    biases: Array1<f32>,
    // Armazena a última entrada e saída para o cálculo do gradiente
    last_input: Option<Array3<f32>>,
    last_output: Option<Array3<f32>>,
}

impl CamadaConvolucional {
    fn new(in_channels: usize, num_filters: usize, filter_height: usize, filter_width: usize) -> Self {
        let dist = Uniform::new(-1.0, 1.0);
        let mut rng = thread_rng();

        let total = num_filters * in_channels * filter_height * filter_width;
        let filters_vec: Vec<f32> = (0..total).map(|_| dist.sample(&mut rng)).collect();
        let filters = Array4::from_shape_vec(
            (num_filters, in_channels, filter_height, filter_width),
            filters_vec,
        )
        .unwrap();

        let biases_vec: Vec<f32> = (0..num_filters).map(|_| dist.sample(&mut rng)).collect();
        let biases = Array1::from(biases_vec);

        CamadaConvolucional {
            num_filters,
            filter_height,
            filter_width,
            in_channels,
            filters,
            biases,
            last_input: None,
            last_output: None,
        }
    }

    /// Realiza a convolução “válida” (sem padding, stride = 1) e aplica a função sigmoide.
    fn forward(&mut self, input: Array3<f32>) -> Array3<f32> {
        self.last_input = Some(input.clone());
        let in_shape = input.dim(); // (channels, height, width)
        let out_height = in_shape.1 - self.filter_height + 1;
        let out_width = in_shape.2 - self.filter_width + 1;
        let mut output = Array3::<f32>::zeros((self.num_filters, out_height, out_width));

        for f in 0..self.num_filters {
            for y in 0..out_height {
                for x in 0..out_width {
                    let mut sum = 0.0;
                    for c in 0..self.in_channels {
                        for i in 0..self.filter_height {
                            for j in 0..self.filter_width {
                                sum += input[[c, y + i, x + j]] * self.filters[[f, c, i, j]];
                            }
                        }
                    }
                    sum += self.biases[f];
                    output[[f, y, x]] = sigmoid(sum);
                }
            }
        }
        self.last_output = Some(output.clone());
        output
    }

    /// Retropropagação para a camada convolucional.
    fn backward(&mut self, d_out: Array3<f32>, learning_rate: f32) -> Array3<f32> {
        let input = self.last_input.as_ref().unwrap();
        let output = self.last_output.as_ref().unwrap();
        let in_shape = input.dim();
        let out_shape = d_out.dim();

        let mut d_filters = Array4::<f32>::zeros(self.filters.dim());
        let mut d_biases = Array1::<f32>::zeros(self.biases.dim());
        let mut d_input = Array3::<f32>::zeros(in_shape);

        for f in 0..self.num_filters {
            for y in 0..out_shape.1 {
                for x in 0..out_shape.2 {
                    let dout_val = d_out[[f, y, x]];
                    let out_val = output[[f, y, x]];
                    let delta = dout_val * sigmoid_derivada_da_saida(out_val);
                    d_biases[f] += delta;
                    for c in 0..self.in_channels {
                        for i in 0..self.filter_height {
                            for j in 0..self.filter_width {
                                d_filters[[f, c, i, j]] += input[[c, y + i, x + j]] * delta;
                                d_input[[c, y + i, x + j]] += self.filters[[f, c, i, j]] * delta;
                            }
                        }
                    }
                }
            }
        }
        self.filters = &self.filters - &(learning_rate * d_filters);
        self.biases = &self.biases - &(learning_rate * d_biases);
        d_input
    }
}

/// ---------------------------------------------------------------------
/// Camada de Max Pooling
/// ---------------------------------------------------------------------
struct MaxPoolingLayer {
    pool_size: usize,
    last_input: Option<Array3<f32>>,
    max_indices: Option<Vec<Array2<(usize, usize)>>>,
}

impl MaxPoolingLayer {
    fn new(pool_size: usize) -> Self {
        MaxPoolingLayer {
            pool_size,
            last_input: None,
            max_indices: None,
        }
    }

    fn forward(&mut self, input: Array3<f32>) -> Array3<f32> {
        self.last_input = Some(input.clone());
        let (channels, height, width) = input.dim();
        let pooled_height = height / self.pool_size;
        let pooled_width = width / self.pool_size;
        let mut output = Array3::<f32>::zeros((channels, pooled_height, pooled_width));
        let mut max_indices = Vec::with_capacity(channels);

        for c in 0..channels {
            let mut indices = Array2::<(usize, usize)>::from_elem((pooled_height, pooled_width), (0, 0));
            for i in 0..pooled_height {
                for j in 0..pooled_width {
                    let mut max_val = std::f32::MIN;
                    let mut max_idx = (0, 0);
                    for m in 0..self.pool_size {
                        for n in 0..self.pool_size {
                            let y = i * self.pool_size + m;
                            let x = j * self.pool_size + n;
                            let val = input[[c, y, x]];
                            if val > max_val {
                                max_val = val;
                                max_idx = (y, x);
                            }
                        }
                    }
                    output[[c, i, j]] = max_val;
                    indices[[i, j]] = max_idx;
                }
            }
            max_indices.push(indices);
        }
        self.max_indices = Some(max_indices);
        output
    }

    fn backward(&mut self, d_out: Array3<f32>, _learning_rate: f32) -> Array3<f32> {
        let input = self.last_input.as_ref().unwrap();
        let (channels, height, width) = input.dim();
        let pooled_height = height / self.pool_size;
        let pooled_width = width / self.pool_size;
        let mut d_input = Array3::<f32>::zeros(input.dim());
        let max_indices = self.max_indices.as_ref().unwrap();

        for c in 0..channels {
            let indices = &max_indices[c];
            for i in 0..pooled_height {
                for j in 0..pooled_width {
                    let (y, x) = indices[[i, j]];
                    d_input[[c, y, x]] += d_out[[c, i, j]];
                }
            }
        }
        d_input
    }
}

/// ---------------------------------------------------------------------
/// Camada Flatten (Achatar)
/// ---------------------------------------------------------------------
struct FlattenLayer {
    input_shape: Option<(usize, usize, usize)>,
}

impl FlattenLayer {
    fn new() -> Self {
        FlattenLayer { input_shape: None }
    }

    fn forward(&mut self, input: Array3<f32>) -> Array1<f32> {
        self.input_shape = Some(input.dim());
        Array::from(input.into_raw_vec())
    }

    fn backward(&mut self, d_out: Array1<f32>, _learning_rate: f32) -> Array3<f32> {
        let shape = self.input_shape.unwrap();
        Array3::from_shape_vec(shape, d_out.to_vec()).unwrap()
    }
}

/// ---------------------------------------------------------------------
/// Camada Totalmente Conectada (Dense)
/// ---------------------------------------------------------------------
struct DenseLayer {
    weights: Array2<f32>, // formato: (output_size, input_size)
    biases: Array1<f32>,   // formato: (output_size)
    last_input: Option<Array1<f32>>,
    last_output: Option<Array1<f32>>,
}

impl DenseLayer {
    fn new(input_size: usize, output_size: usize) -> Self {
        let dist = Uniform::new(-1.0, 1.0);
        let mut rng = thread_rng();

        let total = output_size * input_size;
        let weights_vec: Vec<f32> = (0..total).map(|_| dist.sample(&mut rng)).collect();
        let weights = Array2::from_shape_vec((output_size, input_size), weights_vec)
            .expect("Erro em weights");

        let biases_vec: Vec<f32> = (0..output_size).map(|_| dist.sample(&mut rng)).collect();
        let biases = Array1::from(biases_vec);

        DenseLayer {
            weights,
            biases,
            last_input: None,
            last_output: None,
        }
    }

    fn forward(&mut self, input: Array1<f32>) -> Array1<f32> {
        self.last_input = Some(input.clone());
        let z = self.weights.dot(&input) + &self.biases;
        let activated = z.mapv(sigmoid);
        self.last_output = Some(activated.clone());
        activated
    }

    fn backward(&mut self, d_out: Array1<f32>, learning_rate: f32) -> Array1<f32> {
        let input = self.last_input.as_ref().unwrap();
        let output = self.last_output.as_ref().unwrap();
        let delta = output.mapv(sigmoid_derivada_da_saida) * d_out;
        let d_weights = produto_externo(&delta, input);
        let d_biases = delta.clone();
        let d_input = self.weights.t().dot(&delta);

        self.weights = &self.weights - &(learning_rate * d_weights);
        self.biases = &self.biases - &(learning_rate * d_biases);
        d_input
    }
}

/// ---------------------------------------------------------------------
/// Estrutura da CNN
/// ---------------------------------------------------------------------
struct CNN {
    conv: CamadaConvolucional,
    pool: MaxPoolingLayer,
    flatten: FlattenLayer,
    fc: DenseLayer,
}

impl CNN {
    fn new(
        in_channels: usize,
        conv_filters: usize,
        filter_height: usize,
        filter_width: usize,
        pool_size: usize,
        fc_output: usize,
        input_height: usize,
        input_width: usize,
    ) -> Self {
        let conv = CamadaConvolucional::new(in_channels, conv_filters, filter_height, filter_width);
        let pool = MaxPoolingLayer::new(pool_size);
        let flatten = FlattenLayer::new();
        let conv_out_height = input_height - filter_height + 1;
        let conv_out_width = input_width - filter_width + 1;
        let pool_height = conv_out_height / pool_size;
        let pool_width = conv_out_width / pool_size;
        let flatten_size = conv_filters * pool_height * pool_width;
        let fc = DenseLayer::new(flatten_size, fc_output);
        CNN {
            conv,
            pool,
            flatten,
            fc,
        }
    }

    fn forward(&mut self, input: Array3<f32>) -> Array1<f32> {
        let out_conv = self.conv.forward(input);
        let out_pool = self.pool.forward(out_conv);
        let out_flat = self.flatten.forward(out_pool);
        self.fc.forward(out_flat)
    }

    fn backward(&mut self, d_out: Array1<f32>, learning_rate: f32) {
        let d_fc = self.fc.backward(d_out, learning_rate);
        let d_flatten = self.flatten.backward(d_fc, learning_rate);
        let d_pool = self.pool.backward(d_flatten, learning_rate);
        let _ = self.conv.backward(d_pool, learning_rate);
    }

    fn train(&mut self, input: Array3<f32>, target: Array1<f32>, learning_rate: f32) {
        let output = self.forward(input.clone());
        let loss_grad = &output - &target;
        self.backward(loss_grad, learning_rate);
    }

    /// Retorna os pesos (camada convolucional e fully connected) para salvar.
    fn get_weights(&self) -> CNNWeights {
        CNNWeights {
            conv_filters: self.conv.filters.clone(),
            conv_biases: self.conv.biases.clone(),
            fc_weights: self.fc.weights.clone(),
            fc_biases: self.fc.biases.clone(),
        }
    }

    /// Carrega os pesos na rede.
    fn load_weights(&mut self, weights: &CNNWeights) {
        self.conv.filters = weights.conv_filters.clone();
        self.conv.biases = weights.conv_biases.clone();
        self.fc.weights = weights.fc_weights.clone();
        self.fc.biases = weights.fc_biases.clone();
    }
}

/// ---------------------------------------------------------------------
/// Estrutura para armazenar os pesos que queremos salvar
/// ---------------------------------------------------------------------
#[derive(Serialize, Deserialize)]
struct CNNWeights {
    conv_filters: Array4<f32>,
    conv_biases: Array1<f32>,
    fc_weights: Array2<f32>,
    fc_biases: Array1<f32>,
}

fn save_weights(weights: &CNNWeights, path: &str) -> std::io::Result<()> {
    let serialized = serde_json::to_string(weights).unwrap();
    let mut file = File::create(path)?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}

fn load_weights_from_file(path: &str) -> std::io::Result<CNNWeights> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let weights: CNNWeights = serde_json::from_str(&contents).unwrap();
    Ok(weights)
}

/// ---------------------------------------------------------------------
/// Função para converter um dígito em vetor one-hot de dimensão 10.
/// ---------------------------------------------------------------------
fn one_hot(label: u8) -> Array1<f32> {
    let mut v = vec![0.0f32; 10];
    v[label as usize] = 1.0;
    Array1::from(v)
}

/// ---------------------------------------------------------------------
/// Função auxiliar para converter uma fatia de vetor em imagem 3D (1, 28, 28).
/// ---------------------------------------------------------------------
fn vec_to_image(data: &[f32]) -> Array3<f32> {
    Array::from_shape_vec((1, 28, 28), data.to_vec()).expect("Erro ao criar imagem")
}

fn main() {
    // Caminho onde os pesos serão salvos
    let caminho_pesos = "pesos.json";

    // Carregamento do MNIST (conforme discutido anteriormente)
    println!("Baixando e carregando o MNIST...");
    let Mnist {
        trn_img,
        trn_lbl,
        tst_img,
        tst_lbl,
        ..
    } = MnistBuilder::new()
        .label_format_digit()
        .training_set_length(60_000)
        .test_set_length(10_000)
        .finalize();

    let trn_img: Vec<f32> = trn_img.into_iter().map(|x| x as f32 / 255.0).collect();
    let tst_img: Vec<f32> = tst_img.into_iter().map(|x| x as f32 / 255.0).collect();

    let train_sample_count = 1000;
    let test_sample_count = 200;
    let image_size = 28 * 28;

    let mut train_data = Vec::with_capacity(train_sample_count);
    for i in 0..train_sample_count {
        let start = i * image_size;
        let end = start + image_size;
        let image = vec_to_image(&trn_img[start..end]);
        let label = one_hot(trn_lbl[i]);
        train_data.push((image, label));
    }

    let mut test_data = Vec::with_capacity(test_sample_count);
    for i in 0..test_sample_count {
        let start = i * image_size;
        let end = start + image_size;
        let image = vec_to_image(&tst_img[start..end]);
        let label = one_hot(tst_lbl[i]);
        test_data.push((image, label));
    }

    println!(
        "Treinamento: {} amostras; Teste: {} amostras",
        train_data.len(),
        test_data.len()
    );

    // Configuração da CNN
    let in_channels = 1;
    let input_height = 28;
    let input_width = 28;
    let conv_filters = 8;
    let filter_height = 3;
    let filter_width = 3;
    let pool_size = 2;
    let fc_output = 10;

    let mut cnn = CNN::new(
        in_channels,
        conv_filters,
        filter_height,
        filter_width,
        pool_size,
        fc_output,
        input_height,
        input_width,
    );

    // Se os pesos já estiverem salvos, carregue-os. Caso contrário, treine a rede.
    if Path::new(caminho_pesos).exists() {
        println!("Carregando pesos salvos de '{}'", caminho_pesos);
        let loaded_weights = load_weights_from_file(caminho_pesos).expect("Erro ao carregar pesos");
        cnn.load_weights(&loaded_weights);
    } else {
        println!("Pesos não encontrados. Iniciando treinamento...");
        let epochs = 10;
        let learning_rate = 0.01;

        for epoch in 0..epochs {
            for (img, target) in train_data.iter() {
                cnn.train(img.clone(), target.clone(), learning_rate);
            }

            let mut total_mse = 0.0;
            for (img, target) in test_data.iter() {
                let output = cnn.forward(img.clone());
                let mse = (&output - target).mapv(|x| x * x).sum() / 10.0;
                total_mse += mse;
            }
            total_mse /= test_data.len() as f32;
            println!("Epoch {}: Test MSE = {:.6}", epoch + 1, total_mse);
        }
        println!("Treinamento concluído. Salvando pesos...");
        let weights = cnn.get_weights();
        save_weights(&weights, caminho_pesos).expect("Erro ao salvar pesos");
    }

    // Exemplo de predição para a primeira imagem do conjunto de teste
    let (test_img, test_target) = &test_data[0];
    let output = cnn.forward(test_img.clone());
    println!("\nPredição para a primeira imagem do teste:");
    println!("Saída prevista: {:?}", output);
    println!("Rótulo esperado: {:?}", test_target);
}
