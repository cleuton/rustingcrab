use napi_derive::napi;

#[napi(js_name = "reverseString")]
pub fn reverse_string(s: String) -> String {
    s.chars().rev().collect()
}