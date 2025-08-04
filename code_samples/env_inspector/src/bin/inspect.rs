use env_inspector::detect_scope;

fn main() {
    // Detect the environment scope and print it
    println!("{:#?}", detect_scope());
}
