macro_rules! token_shape {
    ($($token:tt)*) => {
        impl $crate::macros::doc::maybe::Sealed for $($token)* {}

        impl $crate::macros::doc::maybe::ParseShape for $($token)* {
            fn is_valid_hint(input: ParseStream) -> bool {
                input.lookahead1().peek($($token)*)
            }
        }
    };
}
