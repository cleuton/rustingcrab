#[derive(Clone)]
pub struct Carta {
    pub valor: i32, // No blackjack, o valor das cartas é o mesmo que o número de pontos que ela vale, exceto pelas figuras que valem 10 pontos e o Ás que vale 11 pontos.  
    pub naipe: String,
}

impl Carta {
    pub fn new(valor: i32, naipe: String) -> Carta {
        Carta {
            valor,
            naipe,
        }
    }

    pub fn desenhar(&self) -> String {
        let valor_ou_figura = match self.valor {
            1 => "Ás",
            11 => "Valete",
            12 => "Rainha",
            13 => "Rei",
            _ => &self.valor.to_string(),
        };
        format!("{} de {}; ", valor_ou_figura, self.naipe)
    }
}