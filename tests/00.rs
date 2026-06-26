use restricted::prelude::*;

macro_rules! defi_consti {
    ( $short_name:ident:$ty:ty = $value:expr ) => {
        #[doc = "Happy DI"]
        const $short_name: $ty = $value;
    };
}
defi_consti!(DI : bool = false);
const DDI: bool = DI;

def_const!(B: bool = true);
def_const_direct!(U: u8 = 1);

// @TODO have compile_fail test where a submodule from a different file fails
//
//mod sub;

// #[must_use]
fn f() {
    #![allow(unused)]

    //#[deprecated]
    macro_rules! unused {
        () => {
            let unused = ();
        };
    }
    macro_rules! allowed_unused {
        () => {
            #[allow(unused)]
            let unused = ();
        };
    }

    //#[cfg(all(debug_assertions, not(debug_assertions)))]
    {
        {
            #![deny(unused)]
            let _ok_to_be_unused = ();

            // fails to compile - OK:
            //
            //let x = ();

            {
                #![allow(unused)]
                unused!();
            }

            // ok
            allowed_unused!();
        }
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
                //@TODO add token(s):
                let _ = U2!(.);
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
                //@TODO add token(s):
                let _ = U3!(.);
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
    {
        macro_rules! def_use_and_consume {
            () => {

                fn _use_st() {
                    
                def_use! {
                    St, CamelCase,
                    pub struct St {}
                    impl St {
                        fn new() -> Self {
                            Self {}
                        }
                    }
                }
                
                //fn _use_st() {
                    let st: at_use!(St, CamelCase);
                    st = < at_use!(St, CamelCase) >::new();
                }
            };
        }
        def_use_and_consume!();
    }
    /* */
    //bufo_bufo_private_ident_here_dimvxevsdmqmbnuhyptltyqdlnafhdbg= 0;
}

use f as ff;
