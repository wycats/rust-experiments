#![allow(unused)]

use std::path::Path;

use spectrum::{EmitForTest, EmitPlain, EmitResult, StringArena, StringContext, StyledFragment};
use spectrum_macros::frag;

fn plain<'borrow, 'ctx, C>(
    frag: &'borrow StyledFragment<'ctx, C>,
    ctx: &'borrow C,
) -> EmitResult<String>
where
    C: StringContext<'ctx> + 'ctx,
{
    frag.emit_into_string_with(&EmitPlain, ctx)
}

fn color<'borrow, 'ctx, C>(
    frag: &'borrow StyledFragment<'ctx, C>,
    ctx: &'borrow C,
) -> EmitResult<String>
where
    C: StringContext<'ctx> + 'ctx,
{
    frag.emit_into_string_with(&EmitForTest, ctx)
}

macro_rules! test_case {
    ($ctx:expr, ( $($frag:tt)* ) => plain: $plain:tt => colored: $colored:tt) => {
        assert_eq!(&plain(&frag!($($frag)*), &$ctx)?, $plain);
        assert_eq!(&color(&frag!($($frag)*), &$ctx)?, $colored);
    };

    ($ctx:expr, $frag:tt => plain: $plain:tt => colored: $colored:tt) => {
        assert_eq!(&plain(&frag!($frag), &$ctx)?, $plain);
        assert_eq!(&color(&frag!($frag), &$ctx)?, $colored);
    };
}

struct Stringy {
    value: String,
}

impl Stringy {
    fn value(&self) -> &str {
        &self.value
    }
}

#[test]
fn test_line() -> EmitResult {
    let value = ("outer-value",);
    let stringy = Stringy {
        value: "Niko".to_string(),
    };

    let mut arena = StringArena::default();

    test_case!(arena, [Red: "hello"]
        => plain: "hello"
        => colored: "[Red:hello]" );

    test_case!(arena, "hello"
        => plain: "hello"
        => colored: "[normal:hello]" );

    test_case!(arena, ([Red: "hello"] [Green: "world"])
        => plain: "helloworld"
        => colored: "[Red:hello][Green:world]" );

    test_case!(arena, ([Red: "hello"] value.0 [Green: "world"])
        => plain: "helloouter-valueworld"
        => colored: "[Red:hello][normal:outer-value][Green:world]" );

    test_case!(arena, ([Red: "hello"] stringy.value() [Green: "world"])
        => plain: "helloNikoworld"
        => colored: "[Red:hello][normal:Niko][Green:world]" );

    test_case!(arena, ([Red: "hello"] arena.intern(1 + 1) [Green: "world"])
        => plain: "hello2world"
        => colored: "[Red:hello][normal:2][Green:world]" );

    Ok(())
}

#[test]
fn test_block() -> EmitResult {
    let value = ("value-1", "value-2");
    let stringy = Stringy {
        value: "Niko".to_string(),
    };

    let mut arena = StringArena::default();

    test_case!(arena, ( [Red: "hello"] ; [Green: "world"] )
        => plain: "hello\nworld"
        => colored: "[Red:hello]\n[Green:world]" );

    test_case!(arena, ( "hello" ; "world" )
        => plain: "hello\nworld"
        => colored: "[normal:hello]\n[normal:world]" );

    test_case!(arena, ([Red: "hello"] [Green: "world"] ; [Red: "goodbye"] "world")
        => plain: "helloworld\ngoodbyeworld"
        => colored: "[Red:hello][Green:world]\n[Red:goodbye][normal:world]" );

    test_case!(arena, ([Red: "hello"] (value.0) [Green: "world"] ; [Red: "goodbye"] (value.1) [Green: "world"])
        => plain: "hellovalue-1world\ngoodbyevalue-2world"
        => colored: "[Red:hello][normal:value-1][Green:world]\n[Red:goodbye][normal:value-2][Green:world]" );

    test_case!(arena, ([Red: "hello"] stringy.value() [Green: "world"] ; [Red: stringy.value()])
        => plain: "helloNikoworld\nNiko"
        => colored: "[Red:hello][normal:Niko][Green:world]\n[Red:Niko]" );

    test_case!(arena, ([Red: "hello"] arena.intern(1 + 1) [Green: "world"] ; [Red: arena.intern(1 + 1)])
        => plain: "hello2world\n2"
        => colored: "[Red:hello][normal:2][Green:world]\n[Red:2]" );

    Ok(())
}

#[test]
fn ui() {
    let t = trybuild::TestCases::new();

    let ui_tests = Path::new(env!("CRATE_UI_TESTS"));

    eprintln!("ui tests at: {:?}", ui_tests);
    eprintln!(
        "fail tests at: {:?}",
        ui_tests.join("fail/*.rs").display().to_string()
    );

    t.pass(ui_tests.join("pass/*.rs").display().to_string());
    t.compile_fail(ui_tests.join("fail/*.rs").display().to_string());
}
