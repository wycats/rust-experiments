use derive_new::new;
use proc_macro_error::abort;
use quote::ToTokens;
use syn::parse::discouraged::Speculative;
use syn::{
    parse::{Parse, ParseStream},
    Token,
};

use super::{item::DocItem, ParseShape};

#[allow(unused)]
#[derive(Debug, new)]
pub(crate) struct NestedItem {
    arrow: Token![=>],
    body: Box<NestedBody>,
}

sealed!(NestedItem);

impl ParseShape for NestedItem {
    fn is_valid_hint(input: ParseStream) -> bool {
        let next = input.lookahead1();

        next.peek(Token![=>])
    }
}

impl Parse for NestedItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fork = input.fork();

        let arrow: Token![=>] = fork.parse()?;

        match NestedBody::parse(&fork) {
            Ok(body) => {
                input.advance_to(&fork);
                Ok(NestedItem::new(arrow, Box::new(body)))
            }
            Err(err) => abort! {
                err.span(),
                "{}", err
            },
        }
    }
}

impl ToTokens for NestedItem {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self { body, .. } = self;

        let NestedBody { first, last, items } = &**body;

        tokens.extend(quote_using! {
            [spectrum::Nested, spectrum::Group, spectrum::BoxedDoc] => {
                use #BoxedDoc;

                let mut vec = vec![];

                #(
                    vec.push(#items);
                )*

                #Nested::once(#Group::new(vec), #first, #last).boxed()
            }
        })
    }
}

#[allow(unused)]
#[derive(Debug, new)]
pub(crate) struct NestedBody {
    pub(crate) first: DocItem,
    pub(crate) last: DocItem,
    pub(crate) items: Vec<DocItem>,
}

// impl NestedBody {
//     pub(crate) fn maybe(input: ParseStream) -> MaybeParse<Self> {
//         let next = input.lookahead1();

//         if next.peek(syn::token::Paren) {
//             match Self::parse(input) {
//                 Ok(item) => MaybeParse::Success(item),
//                 Err(err) => MaybeParse::Error(err),
//             }
//         } else {
//             MaybeParse::Nope
//         }
//     }
// }

impl Parse for NestedBody {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let first = DocItem::parse(&input)?;
        let mut last: Option<DocItem> = None;

        let mut items: Vec<DocItem> = vec![];

        loop {
            traceln!(false, "checking if empty in nested body?");
            if input.is_empty() {
                traceln!(false, "empty!");
                break;
            }

            traceln!(false, "not empty!");
            let next = DocItem::parse(&input)?;

            if let Some(last) = last.take() {
                items.push(last);
            }

            last = Some(next);
        }

        match last {
            None => abort! {
                input.span(),
                "At least three elements are needed in a nested group (start, end, and body)"
            },

            Some(last) => Ok(NestedBody { first, last, items }),
        }
    }
}
