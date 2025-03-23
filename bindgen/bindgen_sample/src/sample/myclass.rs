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
    pub fn method_bool(&mut self, arg1: bool) -> bool {
        unsafe { self.raw.method_bool(arg1) }
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
    use super::*;
    #[test]
    pub fn method() {
        let mut test = Sample_MyClass::new();
        test.method();
        test.method_bool(true);
    }
    #[test]
    pub fn method_bool() {
        let mut test = Sample_MyClass::new();
        let result = test.method_bool(true);
        assert_eq!(result, true);
    }
}
