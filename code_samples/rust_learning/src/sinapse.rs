use std::fmt;

pub struct Sinapse {
    pub source_node: usize, // Índice do node de origem (em `model.nodes`)
    pub dest_node: usize,   // Índice do node de destino (em `model.nodes`)
    pub weight: f64,        // Peso da sinapse
    pub gradient: f64,      // Gradiente da sinapse
}

impl Sinapse {
    pub fn new(source_node: usize, dest_node: usize, weight: f64) -> Sinapse {
        Sinapse {
            source_node,
            dest_node,
            weight,
            gradient: 0.0,
        }
    }
}

impl PartialEq for Sinapse {
    fn eq(&self, other: &Self) -> bool {
        self.source_node == other.source_node &&
        self.dest_node == other.dest_node
    }
}

impl Eq for Sinapse {}

impl fmt::Display for Sinapse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sinapse: Source node Index: {} destination node Index: {} weight: {} gradient: {}",
        self.source_node, self.dest_node, self.weight, self.gradient)
    }
}