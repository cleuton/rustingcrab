#[unsafe(no_mangle)]
pub extern "C" fn somar(a: i32, b: i32) -> i32 {
    a + b
}