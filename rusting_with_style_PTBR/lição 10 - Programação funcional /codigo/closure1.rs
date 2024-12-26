fn main() {
    let delta = |a: f64, b: f64, c: f64| b * b - 4.0 * a * c;
    let x1 = |a: f64, b: f64, c: f64| (-b + delta(a, b, c)) / (2.0 * a);
    let x2 = |a: f64, b: f64, c: f64| (-b - delta(a, b, c)) / (2.0 * a);
    let a = 1.0;
    let b = 3.0;
    let c = 2.0;
    println!("Delta: {}", delta(a, b, c));
    println!("x1: {}, x2: {}", x1(a, b, c), x2(a, b, c));
}