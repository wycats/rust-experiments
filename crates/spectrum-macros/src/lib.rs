#![cfg_attr(nightly, feature(proc_macro_diagnostic))]

use macros::doc::Doc;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

mod macros;

use crate::macros::fragment::Fragment;

use quote::quote;
use syn::parse_macro_input;

#[proc_macro_error]
#[proc_macro]
pub fn frag(input: TokenStream) -> TokenStream {
    let Fragment { exprs } = parse_macro_input!(input);

    let expanded = quote! {{
        extern crate spectrum;
        use spectrum::{StyledLine, StyledFragment};

        let mut v: Vec<StyledFragment<_>> = Vec::new();

        #(
            v.push(#exprs);
        )*

        let frag: StyledFragment<_> = StyledLine::new(v).into();
        frag
    }};

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn doc(input: TokenStream) -> TokenStream {
    let Doc {} = parse_macro_input!(input);

    let expanded = quote! {{
        ()
    }};

    TokenStream::from(expanded)
}
