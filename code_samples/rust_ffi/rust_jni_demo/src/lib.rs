use jni::objects::JObject;
use jni::sys::{jint, jlong};
use jni::JNIEnv;

#[unsafe(no_mangle)]
pub extern "system" fn Java_Calculator_factorial(
    _env: JNIEnv,
    _this: JObject,
    n: jint
) -> jlong {
    if n < 0 {
        return 0;
    }
    if n > 20 {
        return 0;
    }
    let mut acc: i64 = 1;
    for i in 1..=n as i64 {
        acc = acc.saturating_mul(i);
    }
    acc as jlong
}