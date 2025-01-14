enum FormaPagamento {
    AVista,
    Parcelado { numero_parcelas: i64 },
}


impl FormaPagamento {
    /// Construtor para pagamentos à vista
    pub fn avista() -> Self {
        FormaPagamento::AVista
    }

    /// Construtor seguro para pagamentos parcelados.
    ///
    /// Retorna `Ok(FormaPagamento::Parcelado)` se `parcelas > 0`,
    /// ou `Err(...)` se for inválido.
    pub fn parcelado(parcelas: i64) -> Result<Self, String> {
        if parcelas > 0 {
            Ok(FormaPagamento::Parcelado { numero_parcelas: parcelas })
        } else {
            Err(format!("Número de parcelas inválido: {}", parcelas))
        }
    }
}

struct Pagamento {
    cliente: String,
    valor_a_vista: f32,
    forma_pagamento: FormaPagamento,
}

fn calcular_valor_parcela(pagamento: &Pagamento) -> f32 {
    match pagamento.forma_pagamento {
        FormaPagamento::AVista => 0.0,
        FormaPagamento::Parcelado { numero_parcelas } => pagamento.valor_a_vista / numero_parcelas as f32,
    }
}

fn main() {
    let numero_parcelas = 0;
    //let numero_parcelas = 5;
    match FormaPagamento::parcelado(numero_parcelas) {
        Ok(forma) => {
            let pagto = Pagamento {
                cliente: "Fulano".to_string(),
                valor_a_vista: 1000.0,
                forma_pagamento: forma,
            };
            println!("Valor da parcela: {} - {}", pagto.cliente, calcular_valor_parcela(&pagto));
        }
        Err(e) => println!("Falhou ao criar pagamento parcelado: {}", e),
    }

    let avista = Pagamento {
        cliente: "Ciclano".to_string(),
        valor_a_vista: 2000.0,
        forma_pagamento: FormaPagamento::avista(),
    };

    println!("Compra à vista: {} - {}", avista.cliente, avista.valor_a_vista);
}