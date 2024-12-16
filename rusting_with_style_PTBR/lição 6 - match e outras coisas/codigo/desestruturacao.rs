   
fn main() {
    // arrays
    let v = [1, 4, 6];
    match v {
        [0, 4, 6] => println!("0,4,6!"),
        [1, x, y] => println!("1, {}, {}", x, y), // Vai mostrar esse!
        [2, _, _] => println!("Começa com 2"),
        _ => println!("Nenhum"),
    }

    // tuplas
    let t = (8, 12, 20);
        match t {
        (8, 12, 6) => println!("8,12,6!"),
        (_, 12, z) => println!("Tem 12 no segundo e o terceiro é {}!",z), // Esse!
        _ => println!("Nenhum"),
    }

    // structs
    struct Funcionario {
        matricula: u32,
        idade: u8,
    }
    let f = Funcionario{matricula: 1007, idade: 32};
    match f {
        Funcionario{matricula: 1007, idade: z} => println!("Fulano tem {} anos", z),
        _ => println!("Não é"),
    }
}