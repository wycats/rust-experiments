use derive_new::new;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::Ident;

use crate::macros::fragment::FragmentItem;

use super::{group::GroupItem, group::ParenItem, nested::NestedItem, ParseShape};

#[allow(unused)]
#[derive(Debug)]
pub(crate) enum DocItem {
    Item(FragmentItem),
    List(ListItem),
    Group(GroupItem),
    Nested(NestedItem),
    Either {
        inline: Box<DocItem>,
        block: Box<DocItem>,
    },
    Single(SingleItem),
}

impl Parse for DocItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        try_parse! {
            in input;
            {
                ParenItem => |item| {
                    match item {
                        ParenItem::Group(group) => DocItem::Group(group),
                        ParenItem::Nested(nested) => DocItem::Nested(nested)
                    }
                },

                // GroupItem(item) => DocItem::Group(item),
                SingleItem => |item| DocItem::Single(item),
                FragmentItem => |item| DocItem::Item(item),
            }
            _ => "Expected a document item"
            // try_parse!(GroupItem in input => DocItem::Group);
            // try_parse!(SingleItem in input => DocItem::Single);
            // try_parse!(FragmentItem in input => DocItem::Item);

            // Err(abort! {
            //     input.span(), "Expected a document item"
            // })
        }
    }
}

impl ToTokens for DocItem {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            DocItem::Item(item) => item.to_tokens(tokens),
            DocItem::List(_) => todo!(),
            DocItem::Group(group) => group.to_tokens(tokens),
            DocItem::Nested(nested) => nested.to_tokens(tokens),
            DocItem::Either {
                inline: _inline,
                block: _block,
            } => todo!(),
            DocItem::Single(item) => item.to_tokens(tokens),
        }
    }
}

#[allow(unused)]
#[derive(Debug, new)]
pub(crate) struct ListItem {
    items: Vec<DocItem>,
}

#[derive(Debug)]
pub enum SingleItem {
    Gap,
    GapHint,
    Boundary,
    BoundaryHint,
}

sealed!(SingleItem);

impl ParseShape for SingleItem {
    fn is_valid_hint(input: ParseStream) -> bool {
        let cursor = input.fork().cursor();

        if let Some((ident, _rest)) = cursor.ident() {
            if ident == "SP" || ident == "SP_HINT" || ident == "BK" || ident == "BK_HINT" {
                return true;
            }
        }

        false
    }
}

impl Parse for SingleItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        try_parse! {
            in input;
            {
                Ident => |ident| {
                    if ident == "SP" {
                        SingleItem::Gap
                    } else if ident == "SP_HINT" {
                        SingleItem::GapHint
                    } else if ident == "BK" {
                        SingleItem::Boundary
                    } else if ident == "BK_HINT" {
                        SingleItem::BoundaryHint
                    } else {
                        return Err(input.error("Expected document item"));
                    }
                },
            }

            _ => "not a single item"
        }
    }
}

impl ToTokens for SingleItem {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(match self {
            SingleItem::Gap => quote_using! {
                [spectrum::GAP] => {
                    #GAP()
                }
            },
            SingleItem::GapHint => quote_using! {
                [spectrum::GAP_HINT] => {
                    #GAP_HINT()
                }
            },
            SingleItem::Boundary => quote_using! {
                [spectrum::BOUNDARY] => {
                    #BOUNDARY()
                }
            },
            SingleItem::BoundaryHint => quote_using! {
                [spectrum::BOUNDARY_HINT] => {
                    #BOUNDARY_HINT()
                }
            },
        });
    }
}
