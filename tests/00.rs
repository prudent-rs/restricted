// https://github.com/rust-lang/rust/issues/35121 otherwise use crate never-say-never
//
//#[feature(never_type)]
use restricted::prelude::*;

// @TODO have compile_fail test where a submodule from a different file fails. (and update sub.rs)
//
//mod sub;

mod pin;

// #[must_use]
fn f() {
    #![allow(unused)]
    {
        {
            macro_rules! def_and_use_const_B2 {
                () => {
                    def_const!(B2: bool = true);

                    let _ = at_const!(B2);
                };
            }
            def_and_use_const_B2!();

            macro_rules! def_and_use_const_direct_U2 {
                () => {
                    def_const_direct!(U2: u8 = 1);
                    let _ = at_const!(U2);
                    let _ = U2!(.); //"direct" access
                };
            }
            def_and_use_const_direct_U2!();
        }
        {
            def_static!(B3: bool = true);

            let _ = at_static!(B3);

            {
                def_static_direct!(U3: u8 = 1);
                let _ = at_static!(U3);
                let _ = U3!(.); //"direct" access
            }
        }
    }

    {
        macro_rules! def_and_use_let {
            () => {
                def_let!(l: bool = true);
                //let _ = at_let!(l);
            };
        }
        def_and_use_let!();
    }
    // ------
    {
        macro_rules! def_use_and_consume {
            () => {

                fn _use_st() {

                    def_use_direct! {
                        St, CamelCase;
                        pub struct St {
                            pub field: (),
                        }
                        impl St {
                            pub fn new() -> Self {
                                Self {
                                    field: ()
                                }
                            }
                        }
                    }

                    {
                        let st: at_use!(St, CamelCase);
                        st = < at_use!(St) >::new();
                    }
                    {
                        let st: St!(.);
                        st = < St!(.) >::new();
                    }

                    // Following instantiation is not possible - we need use_with!{...}
                    /*{
                        let st = St!(.) {
                            field: ()
                        };
                    }*/

                    // WE _CANNOT_ invoke `use_with!` inside a function - `super` keyword doesn't
                    // work there!
                    //
                    /*use_with!{
                        St;
                        st_access, // = module name
                        pub type StAlias = St;
                        pub type StAlias2 = St;
                    }
                    let _: StAlias = StAlias::new();
                    let _: StAlias2 = StAlias2::new();*/
                }
            };
        }
        def_use_and_consume!();
    }
}

def_use! {
    HappyStruct;
    pub struct HappyStruct {}
    impl HappyStruct {
        pub fn new() -> Self {
            Self {}
        }
    }
}

use_with! {
    HappyStruct;
    // any identifier unique in the scope where use_with! is invoked
    st_access, // = module name

    pub type StAlias = HappyStruct;
    pub type StAlias2 = HappyStruct;
}

fn _take_st_alias(_: StAlias) {}
fn _take_st_alias2(_: StAlias2) {}

// @TODO examples with explicit lower_case | UPPER_CASE | CamelCase name convention

//----
fn create_pin() {
    //use pin::create_pin_from_pointer;
    let _ = create_pin_from_pointer!(true);
}

struct StructToInferWhenCallingItsMethod {}
impl StructToInferWhenCallingItsMethod {
    const fn method(&self) {}
    const fn safe() {}
}
const _: () = {
    let s = StructToInferWhenCallingItsMethod {};
    //<_>::method(s);
};
//----

trait Methody {}

fn apply() {
    // Problem: Every macro invocation returns a different function - so they _may_ not be able
    // to be compared.
    //
    //let _f = safy!( unsafe-fn-here);
}
//-----

mod safy;

//trait Lifetime<'a> {}

fn fn_ptr_to_trait<'a>(p: fn()) -> impl Fn() {
    p
}

#[test]
fn compare_fn_ptr_and_its_reference() {
    let p = compare_fn_ptr_and_its_reference;
    let r = fn_ptr_to_trait(p);
    let rr = &r;

    //let p = p as usize;
    //
    //let r =
    //let r = &compare_fn_ptr_and_its_reference as *const _ as usize;

    assert!(&r as *const _ as usize == &rr as *const _ as usize);
}
