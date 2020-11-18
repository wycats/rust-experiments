#![allow(non_snake_case)]

use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt};
use tt_call::{tt_call, tt_return};

use quote::quote;
use syn::{bracketed, parse::Parse, parse::ParseStream, token, Expr, ExprLit, Ident, Lit};
use syn::{parse::discouraged::Speculative, Token};

use super::expr::SimpleExpr;

pub(crate) struct Bracketed {
    style: Ident,
    value: Expr,
}

impl ToTokens for Bracketed {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { style, value } = self;

        tokens.extend(quote! {{
            extern crate spectrum;

            use spectrum::{Color, StyledString, StringContext, Style};
            let string = StringContext::styled((#value), Color::#style);
            string.into()
        }})
    }
}

impl Parse for Bracketed {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let style: Ident = input.parse()?;
        let _: token::Colon = input.parse()?;
        let expr: Expr = input.parse()?;

        if input.is_empty() {
            Ok(Bracketed { style, value: expr })
        } else {
            Err(input.error("Unexpected content after bracketed styled fragment"))
        }
    }
}

pub(crate) enum FragmentItem {
    Bracketed(Bracketed),
    String(Expr),
    Expr(SimpleExpr),
    Newline(Token![;]),
}

macro_rules! tail {
    (
        $caller:tt
        path = [{ $head:ident :: $($rest:tt)* }]
    ) => {
        tt_call! {
            macro = [{ tail }]
            path = [{ $($rest)* }]
        }
    };

    (
        $caller:tt
        path = [{ $head:ident }]
    ) => {
        tt_return! {
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
                { let tt_call! { macro = [{ tail }] path = [{ $head $(:: $import)* }] } = quote! { $head $(:: $import)* }; }
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
        let tt_call! { macro = [{ tail }] path = [{ $head $(:: $import)* }] } = quote! { $head $(:: $import)* };

        $rest
    };
}

impl ToTokens for FragmentItem {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            FragmentItem::Bracketed(bracketed) => tokens.append_all(Some(bracketed)),
            FragmentItem::String(expr) => {
                let quoted = quote_using! {
                    [spectrum::StringContext] => {
                        #StringContext::plain(#expr).into()
                    }
                };

                tokens.extend(quoted);
            }
            FragmentItem::Expr(expr) => {
                tokens.extend(quote_using! {
                    [spectrum::StringContext] => {
                        #StringContext::plain(#expr).into()
                    }
                });
            }
            FragmentItem::Newline(_) => tokens.extend(quote_using! {
                [spectrum::StringContext] => {
                    #StringContext::plain("\n").into()
                }
            }),
        }
    }
}

pub(crate) struct Fragment {
    pub(crate) exprs: Vec<FragmentItem>,
}

impl Parse for Fragment {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut exprs: Vec<FragmentItem> = vec![];

        loop {
            if input.is_empty() {
                break;
            }

            let lookahead = input.lookahead1();

            if lookahead.peek(Token![;]) {
                let semi: Token![;] = input.parse().unwrap();
                exprs.push(FragmentItem::Newline(semi));
                continue;
            }

            if lookahead.peek(token::Bracket) {
                let content;
                bracketed!(content in input);
                let bracketed = Bracketed::parse(&content)
                    .map_err(|_| input.error("Wrong content found inside [...]"))?;
                exprs.push(FragmentItem::Bracketed(bracketed));
                continue;
            }

            let fork = input.fork();

            if let Ok(
                expr @ ExprLit {
                    lit: Lit::Str(_), ..
                },
            ) = fork.parse::<ExprLit>()
            {
                exprs.push(FragmentItem::String(Expr::Lit(expr)));
                input.advance_to(&fork);
                continue;
            }

            let expr = fork.parse::<SimpleExpr>()?;
            input.advance_to(&fork);
            exprs.push(FragmentItem::Expr(expr));
        }

        Ok(Fragment { exprs })
    }
}
