use proc_macro2::Span;
use proc_macro_error::abort;
use syn::parse::{discouraged::Speculative, Parse, ParseStream};

pub(crate) enum ParseOutcome<T> {
    /// The parse succeeded, and the parse stream is at the next starting point
    Success(T),
    /// The lookahead condition of the parse succeeded, but then the parse failed, and the
    /// parse stream is now ahead of where it started
    Error(syn::Error),
    /// The lookahead condition of the parse failed, and the parse stream is still at the
    /// original position
    Nope(Span),
}

impl<T> ParseOutcome<T> {
    pub(crate) fn must(self, error: &str) -> syn::Result<T> {
        match self {
            ParseOutcome::Success(value) => Ok(value),
            ParseOutcome::Error(err) => Err(err),
            ParseOutcome::Nope(span) => abort! { span, error },
        }
    }
}

pub(crate) trait Sealed {}

/// The `MaybeParse` trait allows a shape to express the difference between failing to identify
/// a candidate at the input position altogether and identifying a candidate but then failing
/// afterwards.
///
/// In general, when trying multiple `MaybeParse`s in sequence, once a shape satisfies its
/// condition, any subsequent failures will be reported to the user.
///
/// If `maybe_parse` is not implemented, the default implementation will fork the parse stream
/// and advance it only if the parse succeeds.
pub(crate) trait ParseShape: Sized + Parse + Sealed {
    fn is_valid_hint(_input: ParseStream) -> bool {
        true
    }

    fn maybe_parse(input: ParseStream) -> ParseOutcome<Self> {
        if Self::is_valid_hint(input) {
            let fork = input.fork();
            match Self::parse(&fork) {
                Ok(item) => {
                    input.advance_to(&fork);
                    ParseOutcome::Success(item)
                }
                Err(err) => ParseOutcome::Error(err),
            }
        } else {
            ParseOutcome::Nope(input.span())
        }
    }
}

token_shape!(syn::token::Or);

sealed!(syn::Ident);

impl ParseShape for syn::Ident {
    fn is_valid_hint(input: ParseStream) -> bool {
        input.lookahead1().peek(syn::Ident)
    }
}

sealed!(syn::Block);

impl ParseShape for syn::Block {
    fn is_valid_hint(input: ParseStream) -> bool {
        input.lookahead1().peek(syn::token::Brace)
    }
}
