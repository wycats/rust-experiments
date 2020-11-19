// use spectrum_macros::doc;

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
