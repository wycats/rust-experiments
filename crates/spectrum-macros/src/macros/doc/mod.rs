#[macro_use]
mod helper_macros;

mod group;
mod item;
pub(crate) mod maybe;
mod nested;

use syn::parse::{Parse, ParseStream};

pub(crate) use self::maybe::{ParseOutcome, ParseShape};

use self::item::DocItem;

pub(crate) struct Doc {
    pub(crate) items: Vec<DocItem>,
}

sealed!(Doc);

impl ParseShape for Doc {}

impl Parse for Doc {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut items: Vec<DocItem> = vec![];

        loop {
            traceln!(false, "checking if empty?");
            if input.is_empty() {
                traceln!(false, "empty!");
                break;
            }

            traceln!(false, "not empty!");
            items.push(DocItem::parse(&input)?);
        }

        Ok(Doc { items })
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
