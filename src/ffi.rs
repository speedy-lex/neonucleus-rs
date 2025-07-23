#[allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/nn_bindings.rs"));
}

pub use bindings::*;