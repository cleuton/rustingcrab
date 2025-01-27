use sinapse::Sinapse;

#[derive(Debug)]
pub struct Node {
    pub layer_number: usize, // Índice da layer no Model
    pub node_number: usize,  // Índice do node no Model 
    pub sinapses: Vec<uint>, // Sinapses que saem deste node
    pub f64 input;           // Valor de entrada do node
    pub f64 value;           // Valor de saída do node

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.layer_number == other.layer_number &&
        self.node_number == other.node_number
    }
}

impl Eq for Node {}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node: layer Index: {} node Index: {} input: {} value: {}",
        self.layer_number, self.node_number, self.input, self.value);
    }
}