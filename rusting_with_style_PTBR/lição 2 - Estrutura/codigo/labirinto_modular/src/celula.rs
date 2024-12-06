
pub struct Celula {
    pub paredes: [bool; 4],
    pub visitada: bool,
    pub inicio: bool,
    pub fim: bool,
    pub x: usize,
    pub y: usize,
}

pub const NORTE: usize = 0;
pub const SUL: usize = 1;
pub const LESTE: usize = 2;
pub const OESTE: usize = 3;
