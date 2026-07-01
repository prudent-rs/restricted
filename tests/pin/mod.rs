use restricted::prelude::*;

#[macro_export]
macro_rules! create_pin_from_pointer {
    ($pointer:expr) => {{
        def_use_direct! {
            PinRestricted;

            #[repr(transparent)] // plus: #[rustc_pub_transparent] etc.
            pub struct PinRestricted<Ptr> {
                pub pointer: Ptr
            }
        }
        /*use_with! {
            PinRestricted;
            pin_access,

            PinRestricted {
                pointer: $pointer
            }
        }*/
    }};
}
