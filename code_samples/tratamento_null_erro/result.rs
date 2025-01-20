/// Representa um Retângulo com base e altura positivas.
struct Retangulo {
    base: u32,
    altura: u32,
}

impl Retangulo {
    /// Cria um novo retângulo, garantindo que base e altura sejam positivas.
    /// Retorna `Ok(Retangulo)` se as invariantes forem respeitadas, ou `Err(String)` caso contrário.
    fn novo(base: u32, altura: u32) -> Result<Retangulo, String> {
        if base == 0 {
            return Err("A base deve ser maior que zero.".to_string());
        }
        if altura == 0 {
            return Err("A altura deve ser maior que zero.".to_string());
        }
        Ok(Retangulo { base, altura })
    }

    /// Calcula a área do retângulo.
    fn area(&self) -> u32 {
        self.base * self.altura
    }
}

/// Uma função que tenta criar um retângulo e propaga o erro para quem chamou.
fn criar_retangulo(base: u32, altura: u32) -> Result<Retangulo, String> {
    // Propaga qualquer erro de `Retangulo::novo` para cima usando `?`
    let retangulo = Retangulo::novo(base, altura)?;
    Ok(retangulo)
}

fn main() -> Result<(), String> {

    // Tentativa de criar um retângulo válido e usar o operador `?` para propagar erros
    let retangulo = criar_retangulo(5, 10)?;
    println!("Retângulo criado com sucesso: {} x {}", retangulo.base, retangulo.altura);
    println!("Área do retângulo: {}", retangulo.area());

    // Tentativa de criar um retângulo inválido sem verificar o erro: 

    let retangulo_errado = criar_retangulo(0, 10); 
    println!("Retângulo criado, mas na verdade contém um erro!");

    // Tentativa de criar um retângulo inválido testando com if-let: 
    if let Err(erro) = criar_retangulo(0, 10) {
        println!("Erro ao criar retângulo: {}", erro);
    } else {
        println!("Retângulo criado com sucesso!");
    }  

    // Tentativa de criar um retangulo inválido testando com match: 
    match criar_retangulo(0, 10) {
        Ok(retangulo) => println!("Retângulo criado com sucesso: {} x {}", retangulo.base, retangulo.altura),
        Err(erro) => println!("Erro ao criar retângulo: {}", erro),
    }

    // Tentativa de criar um retângulo inválido (propaga o erro automaticamente com `?`)
    let retangulo_errado = criar_retangulo(0, 10)?;
    println!("Retângulo criado: {} x {}", retangulo_errado.base, retangulo_errado.altura);

    Ok(())
}
