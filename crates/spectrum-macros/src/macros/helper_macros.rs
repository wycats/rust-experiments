use syn::parse::ParseStream;

#[macro_use]
macro_rules! sealed {
    ($($tokens:tt)*) => {
        impl $crate::macros::doc::maybe::Sealed for $($tokens)* {}
    }
}

#[macro_use]
macro_rules! traceln {
    (false, $($tokens:tt)*) => { () };
    (true, $($tokens:tt)*) => {
        eprintln!($($tokens)*);
    }
}

#[macro_use]
macro_rules! try_parse {
    ($parser:ty, input: $input:expr) => {{
        try_parse!($parser, input: $input => )
    }};

    ($parser:ty, input: $input:expr => $($constructor:tt)*) => {{
        traceln!(false, "Trying to parse {} @ {:?}", stringify!($parser), $input.span());

        match <$parser>::maybe_parse($input) {
            $crate::macros::doc::ParseOutcome::Success(item) => {
                traceln!(false, "Success {:?}", item);
                return Ok($($constructor)*(item))
            }
            $crate::macros::doc::ParseOutcome::Error(err) => {
                traceln!(false, "Failure {:?}", err);

                return Err(err);
            }
            $crate::macros::doc::ParseOutcome::Nope => {
                traceln!(false, "Not a candidate");
            }
        }
    }};

    (in $input:expr; { $($parser:ty => |$success:pat| $expr:expr, )* } _ => $default:expr) => {{
        use $crate::macros::doc::ParseShape;

        $(
            traceln!(false, "Trying to parse {} @ {:?}", stringify!($parser), $input.span());
            match <$parser>::maybe_parse($input) {
                $crate::macros::doc::ParseOutcome::Success($success) => {
                    let expr = $expr;
                    traceln!(false, "Success {:?}", &expr);
                    return Ok(expr);
                }
                $crate::macros::doc::ParseOutcome::Error(err) => {
                    traceln!(false, "Failure {:?}", err);
                    return Err(err);
                }
                $crate::macros::doc::ParseOutcome::Nope(_) => {
                    traceln!(false, "Not a candidate");
                }
            }
        )*

        traceln!(false, "failed at {:?}", $input.span());
        extern crate proc_macro_error;
        proc_macro_error::abort! { $input.span(), $default }
    }};
}

/// `consume_rest` is used together with `emit_error!` so that once an error is encountered inside
/// a particular delimited area, only that error is reported for that area, but subsequent errors
/// are still reported.
///
/// When using this strategy, it's important to make sure that the error still emits tokens that
/// type check correctly. They don't need to have correct runtime behavior since the error will
/// prevent the code from compiling.
pub(crate) fn consume_rest(input: ParseStream) -> syn::Result<()> {
    input.step(|cursor| {
        let mut rest = *cursor;
        while let Some((_, next)) = rest.token_tree() {
            rest = next;
        }

        Ok(((), rest))
    })
}

macro_rules! parse {
    ($parser:ty, $input:expr => {
        Err($err:ident) => Error {
            message: $error_message:expr,
            fallback: $error_fallback:expr
        },
        Ok($ok:ident) => $ok_expr:expr
    }) => {{
        match <$parser>::parse(&$input) {
            Err($err) => {
                $crate::macros::helper_macros::consume_rest(&$input)?;

                emit_error! { $err.span(), $error_message };
                $error_fallback
            }
            Ok($ok) => $ok_expr,
        }
    }};
}
