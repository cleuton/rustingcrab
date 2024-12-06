use std::env;
use maze_generator::{Labirinto, Solver};

fn main() {
    let args: Vec<String> = env::args().collect();
    let linhas = if args.len() > 1 { args[1].parse::<usize>().unwrap_or(10) } else { 10 };
    let colunas = if args.len() > 2 { args[2].parse::<usize>().unwrap_or(10) } else { 10 };

    let mut labirinto = Labirinto::new(linhas, colunas);
    println!("{}", labirinto);

    let mut solver = Solver::new();
    solver.solve(&mut labirinto);
    println!("{}", labirinto);
}
