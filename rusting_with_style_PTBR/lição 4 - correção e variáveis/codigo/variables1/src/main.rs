use std::any::type_name;

fn tipo_de<T>(_: &T) -> &'static str {
    type_name::<T>()
}

fn nao_altera(x: i32) {
    println!("x = {}", x);
}

fn altera(x: &mut i32) {
    *x += 1;
}

fn main() {
    let mut a = 5; 
    println!("O tipo de a é: {}", tipo_de(&a));
    nao_altera(a);
    altera(&mut a);
    nao_altera(a);
}
