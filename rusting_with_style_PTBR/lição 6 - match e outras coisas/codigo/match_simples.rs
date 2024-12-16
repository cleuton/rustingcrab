#[allow(dead_code)]
enum Dia {
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado,
    Domingo,
}

fn verificar_dia(dia: Dia) {
    match dia {
        Dia::Sabado | Dia::Domingo => {
            println!("É fim de semana!");
        },
        Dia::Segunda | Dia::Terca | Dia::Quarta | Dia::Quinta | Dia::Sexta => {
            println!("É um dia útil.");
        },
    }
}

fn main() {
    verificar_dia(Dia::Terca);
}