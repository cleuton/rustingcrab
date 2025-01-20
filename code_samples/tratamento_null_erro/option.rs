fn calcular_parcela(montante: i32, numero_parcelas: Option<i32>) -> Option<i32> {
    if let Some(parcelas) = numero_parcelas {
        if parcelas == 0 {
            None // Divisão por zero não é possível
        } else {
            Some(montante / parcelas)
        }
    } else {
        None // numero_parcelas é None
    }
}

fn main() {
    let montante = 100;
    let numero_parcelas = Some(0);
    let parcela = calcular_parcela(montante, numero_parcelas);
    match parcela {
        Some(valor) => println!("Valor da parcela: {}", valor),
        None => println!("Pagamento à vista"),
    }

    // Várias outras maneiras de lidar com um resultado "Option":

    if let Some(valor) = calcular_parcela(montante, numero_parcelas) {
        println!("Valor da parcela: {}", valor);
    } else {
        println!("Pagamento à vista");
    }
    
    let parcela = calcular_parcela(montante, numero_parcelas).unwrap_or(0);
    println!("Valor da parcela: {}", parcela);

    let parcela = calcular_parcela(montante, numero_parcelas).unwrap_or_else(|| 0);
    println!("Valor da parcela: {}", parcela);

    let parcela = calcular_parcela(montante, numero_parcelas).expect("Erro ao calcular a parcela"); // Vai dar "panic"

    // let parcela = calcular_parcela(montante, numero_parcelas).unwrap(); // Vai dar "panic"

    //let parcela = calcular_parcela(montante, numero_parcelas)? // Vai repassar o problema para cima. Neste caso vai dar erro.
        
 
}