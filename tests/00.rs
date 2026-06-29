use restricted::prelude::*;

// @TODO have compile_fail test where a submodule from a different file fails. (and update sub.rs)
//
//mod sub;

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

def_use_direct! {
    PinRestricted;

    #[repr(transparent)] // plu: #[rustc_pub_transparent] etc.
    pub struct PinRestricted<Ptr> {
        pub pointer: Ptr
    }
}
