use syn::parse::{Parse, ParseStream};

pub(crate) struct Doc {}

impl Parse for Doc {
    fn parse(_input: ParseStream) -> syn::Result<Self> {
        Ok(Doc {})
    }
}
