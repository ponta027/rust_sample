#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use crate::sample::root::CallBackClass as ffi_myClass;

pub struct Sample_MyClass {
    raw: ffi_myClass,
}
impl Sample_MyClass {
    pub fn new() -> Self {
        unsafe {
            let test = ffi_myClass::new();
            Sample_MyClass { raw: test }
        }
    }
    pub fn method(&mut self) {
        unsafe {
            self.raw.method();
        }
    }

    pub fn method_callback(&mut self, callback: Option<unsafe extern "C" fn(i32)>) {
        unsafe {
            self.raw.method_callback(callback);
        }
    }
    pub fn call_function(&mut self, value: i32) {
        unsafe {
            self.raw.call_function(value);
        }
    }
}
impl Drop for Sample_MyClass {
    fn drop(&mut self) {
        println!("call Drop");
        unsafe {
            self.raw.destruct();
        }
    }
}

#[cfg(test)]
mod tests {
    static mut result: i32 = 0;
    use super::*;
    // Rust のコールバック関数
    extern "C" fn rust_callback(value: i32) {
        println!("Rust callback called with value: {}", value);
        unsafe {
            result = value;
        }
    }

    #[test]
    pub fn method() {
        let mut test = Sample_MyClass::new();
        test.method();
    }
    #[test]
    pub fn method_callback() {
        let mut test = Sample_MyClass::new();
        test.method_callback(Some(rust_callback));
    }
    #[test]
    pub fn call_function() {
        let val = 42;
        let mut test = Sample_MyClass::new();

        test.method_callback(Some(rust_callback));
        test.call_function(val);
        unsafe {
            assert_eq!(result, 42);
        }
    }
}
