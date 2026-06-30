def_use_direct! {
    PinRestricted;

    #[repr(transparent)] // plus: #[rustc_pub_transparent] etc.
    pub struct PinRestricted<Ptr> {
        pub pointer: Ptr
    }
}

use_with! {
    PinRestricted;
    pin_access,

    PinRestricted {

    }
}
