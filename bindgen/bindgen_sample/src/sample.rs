pub mod callback;
pub mod myclass;
pub mod myclass_inherit;
pub mod myclass_ns;
pub mod vector_wrapper;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
