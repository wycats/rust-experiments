macro_rules! tail {
    (
        $caller:tt
        path = [{ $head:ident :: $($rest:tt)* }]
    ) => {
        $crate::tt_call! {
            macro = [{ tail }]
            path = [{ $($rest)* }]
        }
    };

    (
        $caller:tt
        path = [{ $head:ident }]
    ) => {
        $crate::tt_return! {
            $caller
            is = [{ $head }]
        }
    };
}

macro_rules! quote_using {
    ([ $($uses:tt)* ] => $rest:tt) => {{
        quote_using! (
            uses = {} rest = { [ $($uses)*, ] => $rest }
        )
    }};

    (uses = { $({ $($stmt:tt)* })* } rest = { [] => $rest:tt }) => {{
        $(
            $($stmt)*
        )*

        quote::quote! { $rest }
    }};

    (uses = { $($stmt:tt)* } rest = { [ $head:tt $(:: $import:tt)*, $($rest_use:tt)* ] => $rest:tt }) => {
        quote_using! {
            uses = {
                $($stmt)*
                {
                    #[allow(non_snake_case)]
                    let tt_call! { macro = [{ tail }] path = [{ $head $(:: $import)* }] } = quote! { $head $(:: $import)* };
                }
            }
            rest = {
                [ $($rest_use)* ] => $rest
            }
        }
    };

    (uses = { $({ $stmt:stmt })* } rest = { [$head:tt $(:: $import:tt)*] => $rest:tt }) => {
        $(
            $stmt
        )*

        #[allow(non_snake_case)]
        let tt_call! { macro = [{ tail }] path = [{ $head $(:: $import)* }] } = quote! { $head $(:: $import)* };

        $rest
    };
}
