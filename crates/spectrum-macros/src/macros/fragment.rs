#![allow(non_snake_case)]

use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt};
use tt_call::tt_call;

use quote::quote;
use syn::{bracketed, parse::Parse, parse::ParseStream, token, Block, Expr, ExprLit, Ident, Lit};
use syn::{parse::discouraged::Speculative, Token};

pub(crate) enum FragmentItem {
    Bracketed(Bracketed),
    String(Expr),
    Expr(Block),
    Newline(Token![;]),
}

impl Parse for FragmentItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Token![;]) {
            let semi: Token![;] = input.parse().unwrap();
            return Ok(FragmentItem::Newline(semi));
        }

        if lookahead.peek(token::Bracket) {
            let content;
            bracketed!(content in input);
            let bracketed = Bracketed::parse(&content)
                .map_err(|_| input.error("Wrong content found inside [...]"))?;
            return Ok(FragmentItem::Bracketed(bracketed));
        }

        if lookahead.peek(token::Brace) {
            // let content;
            // braced!(content in input);
            let expr = Block::parse(&input)
                .map_err(|_| input.error("Expected a Rust block inside {...}"))?;
            return Ok(FragmentItem::Expr(expr));
        }

        let fork = input.fork();

        if let Ok(
            expr @ ExprLit {
                lit: Lit::Str(_), ..
            },
        ) = fork.parse::<ExprLit>()
        {
            input.advance_to(&fork);
            return Ok(FragmentItem::String(Expr::Lit(expr)));
        }

        Err(input.error("Expected a document fragment"))
    }
}

impl ToTokens for FragmentItem {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            FragmentItem::Bracketed(bracketed) => tokens.append_all(Some(bracketed)),
            FragmentItem::String(expr) => {
                let quoted = quote_using! {
                    [spectrum::plain, spectrum::Doc] => {
                        use #Doc;

                        #plain(#expr).boxed()
                    }
                };

                tokens.extend(quoted);
            }
            FragmentItem::Expr(expr) => {
                tokens.extend(quote_using! {
                    [spectrum::plain, spectrum::Doc] => {
                        use #Doc;

                        #plain(#expr).boxed()
                    }
                });
            }
            FragmentItem::Newline(_) => tokens.extend(quote_using! {
                [spectrum::plain, spectrum::Doc] => {
                    use #Doc;

                    #plain("\n").boxed()
                }
            }),
        }
    }
}

pub(crate) struct Bracketed {
    style: Ident,
    value: Expr,
}

#[allow(unused)]
use spectrum::{plain, styled, Color};

impl ToTokens for Bracketed {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { style, value } = self;

        tokens.extend(quote_using! {
            [spectrum::styled, spectrum::Color, spectrum::Doc] => {
                use #Doc;

                #styled((#value), #Color::#style.into()).boxed()
            }
        })
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

            exprs.push(FragmentItem::parse(&input)?);
        }

        Ok(Fragment { exprs })
    }
}
