use gerador_snowflake::GeradorSnowflake;

fn main() {
    let mut gerador = GeradorSnowflake::novo(1);
    // gera e exibe 5 IDs de exemplo
    for _ in 0..5 {
        println!("{}", gerador.proximo_id());
    }
}
