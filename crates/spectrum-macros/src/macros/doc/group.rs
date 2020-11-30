use derive_new::new;
use quote::ToTokens;
use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    token::Bracket,
};

use super::{item::DocItem, nested::NestedItem, Doc, ParseShape};

#[allow(unused)]
#[derive(Debug, new)]
pub(crate) enum ParenItem {
    Group(GroupItem),
    Nested(NestedItem),
}

sealed!(ParenItem);

impl ParseShape for ParenItem {
    fn is_valid_hint(input: ParseStream) -> bool {
        input.lookahead1().peek(Bracket)
    }
}

impl Parse for ParenItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        bracketed!(content in input);

        try_parse!(
            in &content;
            {
                NestedItem => |item| ParenItem::Nested(item),
                Doc => |doc| ParenItem::Group(GroupItem::new(doc.items)),
            }
            _ => "Invalid content in parens"
        )
    }
}

#[allow(unused)]
#[derive(Debug, new)]
pub(crate) struct GroupItem {
    pub(crate) items: Vec<DocItem>,
}

sealed!(GroupItem);

impl ParseShape for GroupItem {
    fn is_valid_hint(input: ParseStream) -> bool {
        input.lookahead1().peek(syn::token::Bracket)
    }
}

impl Parse for GroupItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let doc = Doc::parse(input);

        Ok(GroupItem { items: doc?.items })
    }
}

impl ToTokens for GroupItem {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self { items } = self;

        tokens.extend(quote_using! {
            [spectrum::Group, spectrum::BoxedDoc] => {
                use #BoxedDoc;

                let mut vec = vec![];

                #(
                    vec.push(#items);
                )*

                #Group::new(vec).boxed()
            }
        })
    }
}
