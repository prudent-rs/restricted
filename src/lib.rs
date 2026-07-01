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

/// @TODO restricted constructor
pub struct Local;

#[repr(transparent)]
struct Unsafe1<'l, R, A1> {
    l: core::marker::PhantomData<&'l ()>,
    p: fn(A1) -> R,
}
impl<'l, R, A1> Unsafe1<'l, R, A1> {
    pub fn unsafe_call(_: A1) -> R {
        todo!()
    }
}

// @TODO async unsafe
//
pub trait UnsafeFnPtr: Copy {
    type Safe;

    type SafeRef<'a>
    where
        Self: 'a;

    //fn unsafey<'a>(self, _: &'a Local) -> Self::SafeRef<'a>;
    fn unsafey<'s: 'l, 'l>(&'s self, _: &'l Local) -> Self::SafeRef<'l>;
}
macro_rules! unsafe_fn_ptr_impl {
    ( $result:ident; $( $arg:ident ),* ) => {

        impl< $result $(, $arg )* > UnsafeFnPtr
        for unsafe fn( $( $arg ),* ) -> $result {

            type Safe = fn( $( $arg ),* ) -> $result;

            type SafeRef<'a> = &'a dyn Fn ( $( $arg ),* ) -> $result where Self: 'a;

            //fn unsafey<'a>(self, _: &'a Local) -> Self::SafeRef<'a> {
            fn unsafey<'s: 'l, 'l>(&'s self, _: &'l Local) -> Self::SafeRef<'l> {
                let ptr: Self::Safe = unsafe { core::mem::transmute(self) };
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
unsafe_fn_ptr_impl!( R; );
unsafe_fn_ptr_impl!( R; A1 );
unsafe_fn_ptr_impl!( R; A1, A2 );
unsafe_fn_ptr_impl!( R; A1, A2, A3 );
unsafe_fn_ptr_impl!( R; A1, A2, A3, A4 );
unsafe_fn_ptr_impl!( R; A1, A2, A3, A4, A5 );
unsafe_fn_ptr_impl!( R; A1, A2, A3, A4, A5, A6 );
unsafe_fn_ptr_impl!( R; A1, A2, A3, A4, A5, A6, A7 );
unsafe_fn_ptr_impl!( R; A1, A2, A3, A4, A5, A6, A7, A8 );
unsafe_fn_ptr_impl!( R; A1, A2, A3, A4, A5, A6, A7, A8, A9 );
unsafe_fn_ptr_impl!( R; A1, A2, A3, A4, A5, A6, A7, A8, A9, A10 );
unsafe_fn_ptr_impl!( R; A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11 );
unsafe_fn_ptr_impl!( R; A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12 );
unsafe_fn_ptr_impl!( R; A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12 ,A13 );

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
fn lifetimed<'a>(arg: &'a ()) -> impl Fn() + 'a {
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

fn lifetimed2<'a>(arg: &'a ()) -> impl Fn() + Lifetimed<'a> {
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
