use textwrap::dedent;

use crate::{
    string::intern::StringArena,
    structure::{
        prelude::*,
        test::{render, render_with},
    },
};

/**
 * ```js
 * function HelloWorld({greeting = "hello", greeted = '"World"', silent = false, onMouseOver,}) {
 *
 * }
 * ```
 */
#[test]
fn test_hello_world() -> TestResult {
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

    let doc = Doc("function ")
        .append("HelloWorld")
        .append(
            Doc("(")
                .append("{")
                .append_group(
                    Group(Doc("greeting").append(" = ").append(r#""hello""#))
                        .append(",")
                        .append(GAP())
                        .append_group(Doc("greeted").append(" = ").append(r#"'"World"'"#))
                        .append(",")
                        .append(GAP())
                        .append_group(Doc("silent").append(" = ").append("false"))
                        .append(",")
                        .append(GAP())
                        .append_group(Doc("onMouseOver"))
                        .append(Alt::inline(EMPTY()).block(","))
                        .wrapping_nest(GAP(), GAP()),
                )
                .append("}")
                .append(")")
                .group(),
        )
        .append(" ")
        .append("{}")
        .append(HARDLINE());

    assert_eq!(render(&doc, &EmitPlain, 80)?, expected_block);
    assert_eq!(render(&doc, &EmitPlain, 96)?, expected_inline);

    Ok(())
}

#[test]
fn test_hello_world_interned() -> TestResult {
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

    let mut strings = StringArena::default();

    let doc = Doc("function ")
        .append(strings.intern("HelloWorld"))
        .append(
            Doc("(")
                .append("{")
                .append_group(
                    Group(Doc("greeting").append(" = ").append(r#""hello""#))
                        .append(",")
                        .append(GAP())
                        .append_group(Doc("greeted").append(" = ").append(r#"'"World"'"#))
                        .append(",")
                        .append(GAP())
                        .append_group(Doc("silent").append(" = ").append("false"))
                        .append(",")
                        .append(GAP())
                        .append_group(Doc("onMouseOver"))
                        .append(Alt::inline(EMPTY()).block(","))
                        .wrapping_nest(GAP(), GAP()),
                )
                .append("}")
                .append(")")
                .group(),
        )
        .append(" ")
        .append("{}")
        .append(HARDLINE());

    assert_eq!(
        render_with(&doc, &EmitPlain, 80, &mut strings)?,
        expected_block
    );
    assert_eq!(
        render_with(&doc, &EmitPlain, 96, &mut strings)?,
        expected_inline
    );

    Ok(())
}

fn strip(input: &str) -> String {
    let lines: Vec<&str> = input.split('\n').collect();
    let string = lines[1..lines.len()].to_vec().join("\n");
    dedent(&string)
}
