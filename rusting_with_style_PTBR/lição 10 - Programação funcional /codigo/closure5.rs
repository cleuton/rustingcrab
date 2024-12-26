struct Calculadora {
    valor: i32,
}

impl Calculadora {
    fn new(valor: i32) -> Calculadora {
        Calculadora { valor }
    }

    fn aplicar<F>(&self, operacao: F) -> i32
    where
        F: Fn(i32) -> i32,
    {
        operacao(self.valor)
    }
}

fn main() {
    let calc = Calculadora::new(10);

    let adicionar = |x| x + 5;
    let multiplicar = |x| x * 2;

    println!("Resultado da adição: {}", calc.aplicar(adicionar));
    println!("Resultado da multiplicação: {}", calc.aplicar(multiplicar));
}