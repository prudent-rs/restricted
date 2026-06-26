pub use {at_const, at_static};
pub use {def_const, def_const_direct, def_static, def_static_direct};

macro_rules! def_const {
    ($short_name:ident : $ty:ty = $value:expr) => {
        // @TODO try to use ::core::stringify! on restricted and #crate - if possible
        #[doc = "(restricted) const"]
        const $short_name: $ty = $value;
    };
}
macro_rules! def_const_direct {
    ($short_name:ident : $ty:ty = $value:expr) => {
        #[doc = "(restricted) const"]
        const $short_name: $ty = $value;

        macro_rules! $short_name {
            ($token_carrier:tt) => {
                $short_name
            };
        }
    };
}
macro_rules! at_const {
    ($short_name:ident) => {
        $short_name
    };
}

macro_rules! def_static {
    ($short_name:ident : $ty:ty = $value:expr) => {
        #[doc = "(restricted) static"]
        static $short_name: $ty = $value;
    };
}
macro_rules! def_static_direct {
    ($short_name:ident : $ty:ty = $value:expr) => {
        #[doc = "(restricted) static"]
        static $short_name: $ty = $value;

        macro_rules! $short_name {
            ($token_carrier:tt) => {
                $short_name
            };
        }
    };
}
macro_rules! at_static {
    ($short_name:ident) => {
        $short_name
    };
}
