
/// Implementação de uma pilha genérica.
pub struct Pilha<T> {
    data: Vec<T>,
}

impl<T> Pilha<T> {
    pub fn new() -> Self {
        Pilha { data: Vec::new() }
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn top(&self) -> Option<&T> {
        self.data.last()
    }
}
