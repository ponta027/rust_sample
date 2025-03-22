#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::sample::root::MyClass as ffi_myClass;

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
    pub fn method_bool(&mut self, arg1: bool) {
        unsafe {
            self.raw.method_bool(arg1);
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
