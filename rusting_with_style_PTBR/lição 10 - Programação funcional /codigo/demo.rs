struct Empregado {
    nome: String,
    salario: f64,
    cargo: String,
}

fn main() {
    let empregados = vec![
        Empregado {
            nome: "João".to_string(),
            salario: 1000.0,
            cargo: "Analista".to_string(),
        },
        Empregado {
            nome: "Maria".to_string(),
            salario: 2000.0,
            cargo: "Gerente".to_string(),
        },
        Empregado {
            nome: "José".to_string(),
            salario: 1500.0,
            cargo: "Analista".to_string(),
        },
        Empregado {
            nome: "Ana".to_string(),
            salario: 3000.0,
            cargo: "Diretor".to_string(),
        },
    ];

    // Uso de closures com filter e map
    let salarios_diretores: Vec<f64> = empregados.iter().filter(|e| e.cargo == "Diretor").map(|e| e.salario).collect();
    println!("Salários dos diretores: {:?}", salarios_diretores);
}