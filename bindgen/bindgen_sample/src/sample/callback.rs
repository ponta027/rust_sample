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
