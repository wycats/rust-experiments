#![cfg_attr(nightly, feature(proc_macro_diagnostic))]

use macros::doc::Doc;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

#[macro_use]
mod macros;

use crate::macros::fragment::Fragment;

use quote::quote;
use syn::parse_macro_input;

pub(crate) use tt_call::{tt_call, tt_return};

#[allow(unused)]
use spectrum::{BoxedDoc, DocList};

#[proc_macro_error]
#[proc_macro]
pub fn frag(input: TokenStream) -> TokenStream {
    let Fragment { exprs } = parse_macro_input!(input);

    let expanded = quote_using! {
        [spectrum::DocList, spectrum::BoxedDoc, spectrum::Doc] => {
            use #Doc;

            // let mut doc = #DocList::new();
            let mut v: Vec<#BoxedDoc> = vec![];

            #(
                v.push(#exprs);
            )*

            #DocList::new(v).boxed()
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_error]
#[proc_macro]
pub fn doc(input: TokenStream) -> TokenStream {
    let Doc { items } = parse_macro_input!(input);

    let expanded = quote_using! {
        [spectrum::BoxedDoc, spectrum::Doc, spectrum::Group] => {
            use #Doc;

            #[allow(unused_mut)]
            let mut vec: Vec<#BoxedDoc> = vec![];

            #(
                vec.push(#items.boxed());
            )*

            #Group::new(vec)
        }
    };

    TokenStream::from(expanded)
}
