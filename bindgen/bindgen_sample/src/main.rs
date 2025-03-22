//use crate::sample::callback;
use crate::sample::callback;
use crate::sample::myclass;
use crate::sample::myclass_inherit;
use crate::sample::myclass_ns;
mod sample;
// Rust のコールバック関数
extern "C" fn rust_callback(value: i32) {
    println!("Rust callback called with value: {}", value);
}

fn main() {
    println!("Hello, world!");

    //
    let mut test = myclass::Sample_MyClass::new();
    test.method();
    test.method_bool(true);

    //
    let mut test = myclass_ns::Sample_MyClass::new();
    test.method();

    //
    let mut test = myclass_inherit::Sample_MyClass::new();
    test.method();

    //
    let mut cbk = callback::Sample_MyClass::new();
    cbk.method_callback(Some(rust_callback));
    cbk.call_function(42);
}
