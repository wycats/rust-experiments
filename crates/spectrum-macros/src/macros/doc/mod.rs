mod group;

use syn::parse::discouraged::Speculative;
use syn::{
    parse::{Parse, ParseStream},
    Ident,
};

use super::fragment::FragmentItem;

pub(crate) struct Doc {}

impl Parse for Doc {
    fn parse(_input: ParseStream) -> syn::Result<Self> {
        Ok(Doc {})
    }
}

#[allow(unused)]
pub(crate) enum DocItem {
    Item(FragmentItem),
    List(ListItem),
    Group(GroupItem),
    Either {
        inline: Box<DocItem>,
        block: Box<DocItem>,
    },
    Single(SingleItem),
}

#[allow(unused)]
pub struct ListItem {
    items: Vec<DocItem>,
}

#[allow(unused)]
pub struct GroupItem {
    items: Vec<DocItem>,
}

pub enum SingleItem {
    Gap,
    GapHint,
    Boundary,
    BoundaryHint,
}

impl Parse for SingleItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fork = input.fork();

        let ident: Ident = fork.parse()?;

        let id = if ident == "GAP" {
            SingleItem::Gap
        } else if ident == "GAP_HINT" {
            SingleItem::GapHint
        } else if ident == "BOUNDARY" {
            SingleItem::Boundary
        } else if ident == "BOUNDARY_HINT" {
            SingleItem::BoundaryHint
        } else {
            return Err(fork.error("Expected document item"));
        };

        input.advance_to(&fork);

        Ok(id)
    }
}

// let doc = list![
//     "function ",
//     "HelloWorld",
//     group![
//         "(",
//         "{",
//         group![
//             group!["greeting", " = ", r#""hello""#],
//             ",",
//             GAP(),
//             group!["greeted", " = ", r#"'"World"'"#],
//             ",",
//             GAP(),
//             group!["silent", " = ", "false"],
//             ",",
//             GAP(),
//             group!["onMouseOver"],
//             either! { inline: empty(), block: "," }
//         ],
//         "}",
//         ")"
//     ]
// ];
