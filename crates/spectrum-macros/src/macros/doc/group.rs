use syn::parse::{Parse, ParseStream};

pub(crate) struct Group {}

impl Parse for Group {
    fn parse(_input: ParseStream) -> syn::Result<Self> {
        todo!()
    }
}
