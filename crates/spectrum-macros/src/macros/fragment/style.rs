use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    token, Block, Ident,
};

use crate::macros::doc::ParseShape;

#[derive(Debug)]
pub enum StyleDescription {
    Color(Ident),
    Expr(Block),
}

sealed!(StyleDescription);

impl ToTokens for StyleDescription {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            StyleDescription::Color(style) => tokens.extend(quote! { (#style).into() }),
            StyleDescription::Expr(expr) => tokens.extend(quote! { (#expr).into() }),
        }
    }
}

impl ParseShape for StyleDescription {
    fn is_valid_hint(input: ParseStream) -> bool {
        let next = input.lookahead1();

        next.peek(Ident) || next.peek(token::Brace)
    }
}

impl Parse for StyleDescription {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        try_parse! {
            in input;
            {
            Ident => |ident| StyleDescription::Color(ident),
            Block => |block| StyleDescription::Expr(block),
            }
            _ => "not a valid style description"
        }
    }
}
