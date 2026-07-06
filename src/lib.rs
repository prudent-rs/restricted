#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(rust_analyzer)]
use no_link as _;
#[cfg(rust_analyzer)]
mod ra_macros;

#[cfg(rust_analyzer)]
use ra_macros as macro_source;
#[cfg(not(rust_analyzer))]
use restricted_enforce as macro_source;

pub mod prelude {
    pub use crate::macro_source::{at_const, at_let, at_mut, at_static, at_use, use_with};
    pub use crate::macro_source::{
        def_const, def_const_direct, def_let, def_let_direct, def_mut, def_mut_direct, def_static,
        def_static_direct, def_use, def_use_direct,
    };
}
pub use prelude::*;

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
//-----------

pub mod fn_item;

/// Parts are based on https://users.rust-lang.org/t/implementing-traits-on-function-pointers/57423/2 > https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=2d064fe8d7f579d0e59df9967861ee7a

// @TODO move to scopy:: and have scopy::local! macro
//
/// @TODO restricted constructor
pub struct Local;

#[repr(transparent)]
struct UnsafeXX<'l, R, A1> {
    l: core::marker::PhantomData<&'l ()>,
    p: fn(A1) -> R,
}
impl<'l, R, A1> UnsafeXX<'l, R, A1> {
    pub fn unsafex(&self, a1: A1) -> R {
        let p = self.p;
        p(a1)
    }
}

/// No `const` support for now. Please state your use case.
//
// @TODO async unsafe
//
pub trait UnsafeFnPtr {
    type AsSafe;

    type AsSafeWrap<'l>
    where
        Self: 'l;

    //fn safy<'s: 'l, 'l>(&'s self, _: &'l Local) -> Self::AsSafeWrap<'l>;
    fn safy<'l>(self, _: &'l Local) -> Self::AsSafeWrap<'l>;
}
macro_rules! unsafe_fn_ptr_impl {
    ( $result:ident, $wrapper_str:ident; $( $arg_name:ident : $arg_ty:ident ),* ) => {

    #[repr(transparent)]
    struct $wrapper_str<'l, R, $( $arg_ty ),* > {
        l: ::core::marker::PhantomData<&'l ()>,
        p: fn( $( $arg_ty ),* ) -> R,
    }
    impl<'l, R,$( $arg_ty ),* > $wrapper_str<'l, R, $( $arg_ty ),* > {
        pub fn unsafex(&self, $( $arg_name:$arg_ty ),* ) -> R {
            let p = self.p;
            p( $( $arg_name ),* )
        }
    }

    impl< $result $(, $arg_ty )* > UnsafeFnPtr
        for unsafe fn( $( $arg_ty ),* ) -> $result {

            type AsSafe = fn( $( $arg_ty ),* ) -> $result;

            type AsSafeWrap<'l> = &'l dyn Fn ( $( $arg_ty ),* ) -> $result where Self: 'l;

            //fn safy<'s: 'l, 'l>(&'s self, _: &'l Local) -> Self::AsSafeWrap<'l> {
            fn safy<'l>(self, _: &'l Local) -> Self::AsSafeWrap<'l> {
                let _ptr: Self::AsSafe = unsafe { core::mem::transmute(self) };
                //unsafe{ core::mem::transmute(ptr) }
                //
                // @TODO:
                //
                // |a1: A1, a2: A3....| f(...)
                todo!()
            }
        }
    }
}
unsafe_fn_ptr_impl!( R, UnsafeFnWrapper00; );
unsafe_fn_ptr_impl!( R, UnsafeFnWrapper01; a1:A1 );
unsafe_fn_ptr_impl!( R, UnsafeFnWrapper02; a1:A1, a2:A2 );
unsafe_fn_ptr_impl!( R, UnsafeFnWrapper03; a1:A1, a2:A2, a3:A3 );
//unsafe_fn_ptr_impl!( R; a1:A1, a2:A2, a3:A3, a4:A4 );
//unsafe_fn_ptr_impl!( R; a1:A1, a2:A2, a3:A3, a4:A4, a5:A5 );
//unsafe_fn_ptr_impl!( R; a1:A1, a2:A2, a3:A3, a4:A4, a5:A5, a6:A6 );
//unsafe_fn_ptr_impl!( R; a1:A1, a2:A2, a3:A3, a4:A4, a5:A5, a6:A6, a7:A7 );

unsafe fn uns_one_arg(_: u8) {}
pub fn invoke_uns_one_arg() {
    //(&uns_one_arg).safy(&Local).unsafex(1);
}

fn ref_to_fn_ptr() {
    fn simple() {}
    let fn_ptr = simple;
    let fn_ptr_ref = &fn_ptr;
    fn_ptr_ref();
}

//-------
//-------
fn simple_fn() {}

//type SSS = struct SSS_ {};
fn lifetimed<'a>(_arg: &'a ()) -> impl Fn() + 'a {
    simple_fn
}

pub fn try_return_lifetimed_to_escape_scope<'a>(arg: &'a ()) -> impl Fn() {
    lifetimed(arg)
}
//-------

trait Lifetimed<'a> {}

impl<'a, T> Lifetimed<'a> for T
where
    T: Fn(),
    T: 'a,
{
}

fn lifetimed2<'a>(_arg: &'a ()) -> impl Fn() + Lifetimed<'a> {
    simple_fn
}

pub fn try_return_lifetimed_to_escape_scope2<'a>(arg: &'a ()) -> impl Fn() + Lifetimed<'a> {
    lifetimed2(arg)
}

pub fn fn_ptr_to_impl<'a, T: Lifetimed<'a>>(_: T, p: fn()) -> impl Fn() + 'a {
    p
}

/*impl<'l> core::ops::Deref for DereFn<'l> {
    /*pub fn unsafy(&self) -> impl Fn() {*/
    type Target = ;
}*/
//-------
//-------

//-------
//-------
//-------
//-------
//-------
