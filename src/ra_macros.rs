#[macro_export]
macro_rules! def_const {
    ($short_name:ident:$ty:ty = $value:expr) => {
        const $short_name:ident:$ty:ty = $value:expr
    };
}
#[macro_export]
macro_rules! at_const {
    () => {};
}
