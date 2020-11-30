#[cfg(test)]
mod tests {
    use console::Color;
    use console::Color::Red;
    use spectrum_macros::doc;

    use spectrum::prelude::test::*;
    use spectrum::{prelude::*, EmitBackendTrait, Style};
    use textwrap::dedent;

    #[test]
    fn compose_leaf_macros() -> TestResult {
        let actual = doc!["Hello" SP "World"];

        assert_eq!(EmitPlain.render(&actual, 25)?, "Hello World");
        assert_eq!(EmitPlain.render(&actual, 5)?, "Hello\nWorld");

        Ok(())
    }

    #[test]
    fn compose_group_macros() -> TestResult {
        // let actual = doc![( "function" "(" ")" )];

        // `BK` means a valid place to break.
        //
        // `SP` means a valid place to break, but if no break occurs, insert a space.
        //
        // `[ => ]` means nest the contents by one additional indentation.

        let actual = doc![
            [ [ "function" BK "(" BK ")" ] SP "{" ]
            [ => SP "Hello" SP "World" SP ]
            "}"
        ];

        assert_eq!(EmitPlain.render(&actual, 80)?, "function() { Hello World }");
        assert_eq!(
            EmitPlain.render(&actual, 13)?,
            "function() {\n  Hello World\n}"
        );

        assert_eq!(
            EmitPlain.render(&actual, 12)?,
            "function() {\n  Hello\n  World\n}"
        );

        assert_eq!(
            EmitPlain.render(&actual, 10)?,
            "function()\n{\n  Hello\n  World\n}"
        );

        assert_eq!(
            EmitPlain.render(&actual, 8)?,
            "function\n(\n)\n{\n  Hello\n  World\n}"
        );

        Ok(())
    }

    struct Punct;

    impl Into<Style> for Punct {
        fn into(self) -> Style {
            Style::default().fg(Color::Black).bold()
        }
    }

    #[test]
    fn compose_group_color_macros() -> TestResult {
        // `BK` means a valid place to break.
        //
        // `GAP` means a valid place to break, but if no break occurs, insert a space.
        //
        // `[ => ]` means nest the contents by one additional indentation. The first and last
        // element of the nesting list will be used to surround the nesting, which is most if you
        // want to decide whether the inline version should have a space or not.

        let actual = doc![
            [ [ [ (Red:"function") BK (Punct:"(") ] BK (Punct:")") ] SP "{" ]
            [ => SP "Hello" SP "World" SP ]
            "}"
        ];

        assert_eq!(
            EmitForTest.render(&actual, 80)?,
            test_emit![
                [Red: "function"] [Black, bold: "("] [Black, bold: ")"]
                SP "{" SP "Hello" SP "World" SP "}"
            ]
        );

        assert_eq!(
            EmitForTest.render(&actual, 13)?,
            test_emit![
                | [Red: "function"] [Black, bold: "("] [Black, bold: ")"] SP "{"
                | => "Hello" SP "World"
                | "}"
            ]
        );

        assert_eq!(
            EmitForTest.render(&actual, 12)?,
            test_emit![
                | [Red: "function"] [Black, bold: "("] [Black, bold: ")"] SP "{"
                | => "Hello"
                | => "World"
                | "}"
            ]
        );

        assert_eq!(
            EmitForTest.render(&actual, 10)?,
            test_emit![
                | [Red: "function"] [Black, bold: "("] [Black, bold: ")"]
                | "{"
                | => "Hello"
                | => "World"
                | "}"
            ]
        );

        assert_eq!(
            EmitForTest.render(&actual, 9)?,
            test_emit![
                | [Red: "function"] [Black, bold: "("]
                | [Black, bold: ")"]
                | "{"
                | => "Hello"
                | => "World"
                | "}"
            ]
        );

        assert_eq!(
            EmitForTest.render(&actual, 8)?,
            test_emit![
                | [Red: "function"]
                | [Black, bold: "("]
                | [Black, bold: ")"]
                | "{"
                | => "Hello"
                | => "World"
                | "}"
            ]
        );

        Ok(())
    }

    #[test]
    fn compose_group_color_macros_hints() -> TestResult {
        // `BK` means a valid place to break.
        //
        // `GAP` means a valid place to break, but if no break occurs, insert a space.
        //
        // `[ => ]` means nest the contents by one additional indentation. The first and last
        // element of the nesting list will be used to surround the nesting, which is most if you
        // want to decide whether the inline version should have a space or not.

        let actual = doc![
            [ (Red:"function") BK_HINT (Punct:"(") BK_HINT (Punct:")") SP "{" ]
            [ => SP "Hello" SP "World" SP ]
            "}"
        ];

        assert_eq!(
            EmitForTest.render(&actual, 80)?,
            test_emit![
                [Red: "function"] [Black, bold: "("] [Black, bold: ")"]
                SP "{" SP "Hello" SP "World" SP "}"
            ]
        );

        assert_eq!(
            EmitForTest.render(&actual, 13)?,
            test_emit![
                | [Red: "function"] [Black, bold: "("] [Black, bold: ")"] SP "{"
                | => "Hello" SP "World"
                | "}"
            ]
        );

        assert_eq!(
            EmitForTest.render(&actual, 12)?,
            test_emit![
                | [Red: "function"] [Black, bold: "("] [Black, bold: ")"] SP "{"
                | => "Hello"
                | => "World"
                | "}"
            ]
        );

        assert_eq!(
            EmitForTest.render(&actual, 10)?,
            test_emit![
                | [Red: "function"] [Black, bold: "("] [Black, bold: ")"]
                | "{"
                | => "Hello"
                | => "World"
                | "}"
            ]
        );

        assert_eq!(
            EmitForTest.render(&actual, 9)?,
            test_emit![
                | [Red: "function"] [Black, bold: "("]
                | [Black, bold: ")"]
                | "{"
                | => "Hello"
                | => "World"
                | "}"
            ]
        );

        assert_eq!(
            EmitForTest.render(&actual, 8)?,
            test_emit![
                | [Red: "function"]
                | [Black, bold: "("] [Black, bold: ")"]
                | "{"
                | => "Hello"
                | => "World"
                | "}"
            ]
        );

        Ok(())
    }

    // #[test]
    // fn compose_smoke_macros() -> TestResult {
    //     let expected_block = strip(
    //         r#"
    //         function HelloWorld({
    //           greeting = "hello",
    //           greeted = '"World"',
    //           silent = false,
    //           onMouseOver,
    //         }) {}
    //     "#,
    //     );

    //     let expected_inline = "function HelloWorld({ greeting = \"hello\", greeted = '\"World\"', silent = false, onMouseOver }) {}";

    //     let doc = list![
    //         "function ",
    //         "HelloWorld",
    //         group![
    //             "(",
    //             "{",
    //             nest![
    //                 {
    //                     group!["greeting", " = ", r#""hello""#],
    //                     ",",
    //                     GAP(),
    //                     group!["greeted", " = ", r#"'"World"'"#],
    //                     ",",
    //                     GAP(),
    //                     group!["silent", " = ", "false"],
    //                     ",",
    //                     GAP(),
    //                     group!["onMouseOver"],
    //                     either! { inline: empty(), block: "," }
    //                 }
    //                 before = GAP();
    //                 after = GAP();
    //             ],
    //             "}",
    //             ")",
    //             " ",
    //             "{}"
    //         ]
    //     ];

    //     assert_eq!(EmitPlain.render(&doc, 80)?, expected_block);
    //     assert_eq!(EmitPlain.render(&doc, 96)?, expected_inline);

    //     Ok(())
    // }

    // fn render(text: &BoxedDoc, page_size: usize) -> Result<String, std::fmt::Error> {
    //     Buf::collect_string(|writer| {
    //         let mut context = RenderContext::new(writer);
    //         context.render(text, EmitPlain, RenderConfig::width(page_size))?;

    //         Ok(())
    //     })
    // }

    #[allow(unused)]
    fn strip(input: &str) -> String {
        let lines: Vec<&str> = input.split('\n').collect();
        let string = lines[1..lines.len() - 1].to_vec().join("\n");
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
