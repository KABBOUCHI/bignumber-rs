#[macro_export]
macro_rules! to_bn {
    ($exp: literal) => {
        $crate::BigNumber::of($exp)
    };
}

#[macro_export]
macro_rules! to_bn_safe {
    ($exp: literal) => {
        $crate::BigNumber::of($exp).unwrap()
    };
}
