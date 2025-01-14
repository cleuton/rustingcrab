struct Pagamento {
    cliente: String,
    valor_a_vista: f32,
    parcelado: bool,
    numero_parcelas: i64,
}

fn calcular_valor_parcela(pagamento: &Pagamento) -> f32 {
    if pagamento.parcelado {
        return pagamento.valor_a_vista / pagamento.numero_parcelas as f32
    }
    0.0
}

fn main() {
    let pagto = Pagamento {
        cliente: "Fulano".to_string(),
        valor_a_vista: 1000.00,
        parcelado: true,
        numero_parcelas: 0,
    };
    println!("Valor da parcela: {} - {}", pagto.cliente, calcular_valor_parcela(&pagto));
}