#[macro_use]
macro_rules! test_emit {
    ( | $($tokens:tt)* ) => {
        test_emit_impl! {
            next { out = { "" } }
            rest = { $($tokens)* }
        }
    };

    ( $($tokens:tt)* ) => {
        test_emit_impl! {
            next { out = { "" } }
            rest = { $($tokens)* }
        }
    };
}

#[macro_use]
macro_rules! test_emit_impl {
    (
        next { out = $({ $($out:tt)* })* }
        rest = {}
    ) => {
        concat![ $( $($out)* ),* ]
    };

    (
        next { out = $out:tt }
        rest = { $token:tt $($rest:tt)* }
    ) => {
        test_emit_impl! {
            token { $token }
            out = $out
            rest = { $($rest)* }
        }
    };

    (
        token { [ $($tokens:tt)* ] }
        out = { $($out:tt)* }
        rest = $rest:tt
    ) => {
        test_emit_impl! {
            next {
                out = {
                    concat![$($out)*, test_emit_impl! { style { color = () on = () rest = { $($tokens)* } } }]
                }
            }
            rest = $rest
        }
    };

    (
        token { => }
        out = { $($out:tt)* }
        rest = $rest:tt
    ) => {
        test_emit_impl! {
            next {
                out = {
                    concat![$($out)*, "[normal:  ]"]
                }
            }
            rest = $rest
        }
    };

    (
        token { | }
        out = { $($out:tt)* }
        rest = $rest:tt
    ) => {
        test_emit_impl! {
            next {
                out = {
                    concat![$($out)*, "\n"]
                }
            }
            rest = $rest
        }
    };

    (
        token { SP }
        out = { $($out:tt)* }
        rest = $rest:tt
    ) => {
        test_emit_impl! {
            next {
                out = {
                    concat![$($out)*, "[normal: ]"]
                }
            }
            rest = $rest
        }
    };

    (
        token { $token:tt }
        out = { $($out:tt)* }
        rest = $rest:tt
    ) => {
        test_emit_impl! {
            next {
                out = {
                    concat![$($out)*, "[normal:", $token, "]"]
                }
            }
            rest = $rest
        }
    };

    (
        style {
            color = ()
            on = ()
            rest = { $color:ident $($tokens:tt)* }
        }
    ) => {
        test_emit_impl!( style { color = $color on = () rest = { $($tokens)* } } )
    };

    (
        style {
            color = $color:ident
            on = ()
            rest = { on $on:ident $($tokens:tt)* }
        }
    ) => {
        test_emit_impl!(
            styled { style = { concat![ stringify![ $color ], " on ", $on ] } rest = { $($tokens)* } }
        )
    };

    (
        style {
            color = $color:ident
            on = ()
            rest = { $($tokens:tt)* }
        }
    ) => {
        test_emit_impl!(
            styled {
                style = { stringify![ $color ] }
                rest = { $($tokens)* }
            }
        )
    };

    (
        styled {
            style = { $($style:tt)* }
            rest = { , $attr:ident $($tokens:tt)* }
        }
    ) => {
        test_emit_impl!(
            styled {
                style = { concat![ $($style)*, ", ", stringify![$attr] ] }
                rest = { $($tokens)* }
            }
        )
    };

    (
        styled {
            style = { $($style:tt)* }
            rest = { : $literal:tt }
        }
    ) => {
        concat![ "[", $($style)*, ":", $literal, "]" ]
    };



}
