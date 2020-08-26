#![allow(clippy::suspicious_op_assign_impl, clippy::suspicious_arithmetic_impl)]
#![deny(
    clippy::clone_on_ref_ptr,
    clippy::dbg_macro,
    clippy::default_trait_access,
    clippy::enum_glob_use,
    clippy::get_unwrap,
    clippy::macro_use_imports,
    clippy::missing_errors_doc
)]

pub mod fnv;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
