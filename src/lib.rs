#[cfg(rust_analyzer)]
const _TODO: () = ();
//@TODO:
/*
#[cfg(rust_analyzer)]
use no_link as _;
*/

#[cfg(not(rust_analyzer))]
const _TODO: () = ();


#[cfg(not(rust_analyzer))]
pub mod prelude {
    pub use restricted_enforce::{def_const, def_const_direct, def_static, def_static_direct};
    pub use restricted_enforce::{at_const, at_static};
    //pub use crate::{def_let, def_let_direct, def_mut, def_mut_direct, def_const, def_const_direct, def_static, def_static_direct, at_let, at_mut, at_const, at_static};
}
/*
/// We can't report the actual version(s), because [panic] macro is not eager, and passing in
/// (formatting) variables doesn't work in `const` context. See also
/// <https://rustc-dev-guide.rust-lang.org/macro-expansion.html#eager-expansion>.
const _: () = {
    let proc_version = restricted_enforce::version!();

    if !is_exact_version(proc_version) {
        panic!(
            "prudent-rs/restricted_enforce is of different version than \
                prudent-rs/restricted. Please report this as an issue, along with both \
                versions."
        );
    }
};

const fn is_exact_version(expected_version: &'static str) -> bool {
    // We can't use a comparison operator ==, because trait PartialEq is not const (in April 2026).
    matches!(expected_version.as_bytes(), b"0.0.1")
}

const _: () = {
    if !is_exact_version(env!("CARGO_PKG_VERSION")) {
        panic!("prudent-rs/restricted has its function is_exact_version() out of date.");
    }
};
*/