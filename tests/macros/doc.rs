// use spectrum_macros::doc;

#[cfg(test)]
mod tests {
    use spectrum::{either, empty, prelude::*, EmitBackendTrait};
    use spectrum::{group, list};
    use spectrum::{prelude::test::*, GAP};
    use textwrap::dedent;

    #[test]
    fn compose_smoke() -> TestResult {
        let expected_block = strip(
            r#"
            function HelloWorld({
              greeting = "hello",
              greeted = '"World"',
              silent = false,
              onMouseOver,
            }) {}
        "#,
        );

        let expected_inline = "function HelloWorld({ greeting = \"hello\", greeted = '\"World\"', silent = false, onMouseOver }) {}\n";

        let doc = list![
            "function ",
            "HelloWorld",
            group![
                "(",
                "{",
                group![
                    group!["greeting", " = ", r#""hello""#],
                    ",",
                    GAP(),
                    group!["greeted", " = ", r#"'"World"'"#],
                    ",",
                    GAP(),
                    group!["silent", " = ", "false"],
                    ",",
                    GAP(),
                    group!["onMouseOver"],
                    either! { inline: empty(), block: "," }
                ],
                "}",
                ")"
            ]
        ];

        assert_eq!(EmitPlain.render(&doc, 80)?, expected_block);
        assert_eq!(EmitPlain.render(&doc, 96)?, expected_inline);

        Ok(())
    }

    // fn render(text: &BoxedDoc, page_size: usize) -> Result<String, std::fmt::Error> {
    //     Buf::collect_string(|writer| {
    //         let mut context = RenderContext::new(writer);
    //         context.render(text, EmitPlain, RenderConfig::width(page_size))?;

    //         Ok(())
    //     })
    // }

    fn strip(input: &str) -> String {
        let lines: Vec<&str> = input.split('\n').collect();
        let string = lines[1..lines.len()].to_vec().join("\n");
        dedent(&string)
    }
}

// let doc = Doc("function ")
// .append("HelloWorld")
// .append(
//     Doc("(")
//         .append("{")
//         .append_group(
//             Group(Doc("greeting").append(" = ").append(r#""hello""#))
//                 .append(",")
//                 .append(GAP())
//                 .append_group(Doc("greeted").append(" = ").append(r#"'"World"'"#))
//                 .append(",")
//                 .append(GAP())
//                 .append_group(Doc("silent").append(" = ").append("false"))
//                 .append(",")
//                 .append(GAP())
//                 .append_group(Doc("onMouseOver"))
//                 .append(Alt::inline(EMPTY()).block(","))
//                 .wrapping_nest(GAP(), GAP()),
//         )
//         .append("}")
//         .append(")")
//         .group(),
// )
// .append(" ")
// .append("{}")
// .append(HARDLINE());

// #[test]
// fn test_doc() {
//     doc! {
//         "function " "HelloWorld" "(" ")"
//     }
// }
