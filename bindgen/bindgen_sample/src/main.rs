#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::root::test;
use crate::root::MyClass;
use crate::root::MyClassInherit;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    println!("Hello, world!");
    unsafe {
        let mut test = MyClass::new();
        test.method();
        test.method_bool(true);
    }
    unsafe {
        let mut test = test::MyClass::new();
        test.method();
    }
    unsafe {
        let mut test = MyClassInherit::new();
        test.method();
    }
}
