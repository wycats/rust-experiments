use proc_macro2::TokenStream;
use quote::ToTokens;

use proc_macro_error::abort;
use quote::quote;
use syn::{
    parenthesized, parse::Parse, parse::ParseStream, punctuated::Punctuated, token::Paren, Expr,
    Ident, Index, Member,
};
use syn::{parse::discouraged::Speculative, Token};

pub struct BareWord {}
