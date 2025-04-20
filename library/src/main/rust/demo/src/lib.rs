use jni::{objects::JClass, sys::jint, JNIEnv};

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn Java_ix_radon_rustydroid_NativeApi_increment(
    _env: JNIEnv,
    _this: JClass,
    mut _i: jint
) {
    _i += 1;
}
