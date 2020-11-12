use proc_macro2::TokenStream;
use quote::ToTokens;

use proc_macro_error::abort;
use quote::quote;
use syn::{
    parenthesized, parse::Parse, parse::ParseStream, punctuated::Punctuated, token::Paren, Expr,
    Ident, Index, Member,
};
use syn::{parse::discouraged::Speculative, Token};

#[derive(Debug)]
pub struct Parenthesized {
    paren: Paren,
    expr: Expr,
}

impl Parse for Parenthesized {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let paren = parenthesized!(content in input);

        match content.parse::<Expr>() {
            Ok(expr) => Ok(Parenthesized { paren, expr }),
            Err(err) => {
                abort! {
                    err.span(),
                    "Expected an expression inside of parens"
                }
            }
        }
    }
}

impl ToTokens for Parenthesized {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { expr, .. } = self;

        tokens.extend(quote! { (#expr) });
    }
}

#[derive(Debug)]
pub struct MethodCall {
    base: Box<SimpleExpr>,
    dot: Token![.],
    field: Ident,
    paren: Paren,
    args: Punctuated<Expr, Token![,]>,
}

#[derive(Debug)]
pub struct TupleField {
    base: Box<SimpleExpr>,
    dot: Token![.],
    field: Index,
}

#[derive(Debug)]
pub struct StructField {
    base: Box<SimpleExpr>,
    dot: Token![.],
    field: Ident,
}

impl StructField {
    fn next(self, input: ParseStream) -> syn::Result<ExprContinuation> {
        let fork = input.fork();

        let content;
        let paren = parenthesized!(content in fork);

        let args = match Punctuated::<Expr, Token![,]>::parse_terminated(&content) {
            Ok(args) => args,
            Err(err) => abort! { err.span(), "method call's arguments were invalid: {}", err },
        };

        let Self { base, dot, field } = self;

        input.advance_to(&fork);
        Ok(ExprContinuation::Next(SimpleExpr::MethodCall(MethodCall {
            base,
            dot,
            field,
            paren,
            args,
        })))
        // Ok(ExprContinuation::Next(SimpleExpr::MethodCall {
        //     base: Box::new(self),
        // }))
    }
}

#[derive(Debug)]
pub enum SimpleExpr {
    Ident(Ident),
    StructField(StructField),
    TupleField(TupleField),
    MethodCall(MethodCall),
    Parenthesized(Parenthesized),
}

#[derive(Debug)]
pub enum ExprContinuation {
    Done(SimpleExpr),
    Next(SimpleExpr),
}

impl SimpleExpr {
    fn parse_field(self, input: &mut ParseStream) -> syn::Result<ExprContinuation> {
        let start = input.span();

        let dot = match input.parse::<Token![.]>() {
            Err(_) => unreachable!("Peek for a `.` token before calling parse_field"),
            Ok(dot) => dot,
        };

        match input.parse::<Member>() {
            Err(err) => {
                abort! {
                    start.join(err.span()).unwrap_or(start),
                    "a dot in a simple expression must be immediately followed by a member (a struct or tuple field access)"
                }
            }
            Ok(Member::Named(id)) => Ok(ExprContinuation::Next(SimpleExpr::StructField(
                StructField {
                    base: Box::new(self),
                    dot,
                    field: id,
                },
            ))),
            Ok(Member::Unnamed(id)) => {
                Ok(ExprContinuation::Next(SimpleExpr::TupleField(TupleField {
                    base: Box::new(self),
                    dot,
                    field: id,
                })))
            }
        }
    }

    fn next(self, input: &mut ParseStream) -> syn::Result<ExprContinuation> {
        let lookahead = input.lookahead1();

        match self {
            expr @ SimpleExpr::Parenthesized(_) => Ok(ExprContinuation::Done(expr)),

            expr @ SimpleExpr::Ident(_) => {
                if lookahead.peek(Token![.]) {
                    expr.parse_field(input)
                } else {
                    Ok(ExprContinuation::Done(expr))
                }
            }
            SimpleExpr::StructField(field) => {
                if lookahead.peek(Token![.]) {
                    SimpleExpr::StructField(field).parse_field(input)
                } else if lookahead.peek(Paren) {
                    field.next(input)
                } else {
                    Ok(ExprContinuation::Done(SimpleExpr::StructField(field)))
                }
            }
            expr @ SimpleExpr::TupleField(_) => {
                if lookahead.peek(Token![.]) {
                    expr.parse_field(input)
                } else {
                    Ok(ExprContinuation::Done(expr))
                }
            }
            expr @ SimpleExpr::MethodCall(_) => {
                if lookahead.peek(Token![.]) {
                    expr.parse_field(input)
                } else {
                    Ok(ExprContinuation::Done(expr))
                }
            }
        }
    }
}

impl Parse for SimpleExpr {
    fn parse(mut input: ParseStream) -> syn::Result<Self> {
        if input.lookahead1().peek(Paren) {
            let paren: Parenthesized = input.parse()?;

            return Ok(SimpleExpr::Parenthesized(paren));
        }

        if input.lookahead1().peek(Ident) {
            let mut expr = SimpleExpr::Ident(input.parse().unwrap());

            loop {
                let next = expr.next(&mut input);

                match next? {
                    ExprContinuation::Done(expr) => return Ok(expr),
                    ExprContinuation::Next(next) => {
                        expr = next;
                    }
                }
            }
        }

        abort!(input.span(), "Simple expression not found")
    }
}

impl ToTokens for SimpleExpr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            SimpleExpr::Ident(ident) => quote! { #ident },
            SimpleExpr::StructField(StructField { base, dot, field }) => quote! {
                #base #dot #field
            },
            SimpleExpr::TupleField(TupleField { base, dot, field }) => quote! {
                #base #dot #field
            },
            SimpleExpr::MethodCall(MethodCall {
                base,
                dot,
                field,
                args,
                ..
            }) => quote! {
                #base #dot #field ( #args )
            },
            SimpleExpr::Parenthesized(expr) => quote! { #expr },
        })
    }
}
