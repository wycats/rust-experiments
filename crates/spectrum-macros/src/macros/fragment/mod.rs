#![allow(non_snake_case)]

mod style;

use proc_macro2::TokenStream;
use proc_macro_error::emit_error;
use quote::{ToTokens, TokenStreamExt};

use style::StyleDescription;
use syn::{parenthesized, Token};
use syn::{parse::Parse, parse::ParseStream, token, Block, Expr, ExprLit, Ident, Lit, LitStr};

#[derive(Debug)]
pub(crate) enum FragmentItem {
    Bracketed(Bracketed),
    String(Expr),
    Expr(Block),
    Newline(Token![;]),
    Error,
}

sealed!(FragmentItem);

impl ParseShape for FragmentItem {
    fn is_valid_hint(input: ParseStream) -> bool {
        let lookahead = input.lookahead1();

        lookahead.peek(Token![;])
            || lookahead.peek(token::Paren)
            || lookahead.peek(token::Brace)
            || lookahead.peek(LitStr)
    }
}

impl Parse for FragmentItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Token![;]) {
            let semi: Token![;] = input.parse().unwrap();
            return Ok(FragmentItem::Newline(semi));
        }

        if lookahead.peek(token::Paren) {
            let content;
            parenthesized!(content in input);

            return Ok(parse!(Bracketed, content => {
                Err(err) => Error {
                    message: "Wrong content found inside [...]",
                    fallback: FragmentItem::Error
                },
                Ok(bracketed) => FragmentItem::Bracketed(bracketed)
            }));
        }

        if lookahead.peek(token::Brace) {
            // let content;
            // braced!(content in input);
            let expr = Block::parse(&input)
                .map_err(|_| input.error("Expected a Rust block inside {...}"))?;
            return Ok(FragmentItem::Expr(expr));
        }

        if lookahead.peek(LitStr) {
            if let Ok(
                expr @ ExprLit {
                    lit: Lit::Str(_), ..
                },
            ) = input.parse::<ExprLit>()
            {
                return Ok(FragmentItem::String(Expr::Lit(expr)));
            }
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
            FragmentItem::Error => tokens.extend(quote_using! {
                [spectrum::plain, spectrum::Doc] => {
                    use #Doc;

                    #plain("[ERROR]").boxed()
                }
            }),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Bracketed {
    style: StyleDescription,
    value: Expr,
}

#[allow(unused)]
use spectrum::{plain, styled, Color};

use super::doc::ParseShape;

impl ToTokens for Bracketed {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { style, value } = self;

        tokens.extend(quote_using! {
            [spectrum::styled, spectrum::Doc] => {
                use #Doc;

                #styled((#value), #style).boxed()
            }
        })
    }
}

impl Parse for Bracketed {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let style = parse!(StyleDescription, input => {
            Err(err) => Error {
                message: "invalid style description",
                fallback: StyleDescription::Color(Ident::new("Red", err.span()))
            },
            Ok(desc) => desc
        });
        // let style: Ident = input.parse()?;
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

            exprs.push(FragmentItem::maybe_parse(&input).must("expected fragment")?);
        }

        Ok(Fragment { exprs })
    }
}
