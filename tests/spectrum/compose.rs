use std::borrow::Cow;

use spectrum::{
    BoxedDoc, Doc, Intern, InternedBoxedDoc, InternedStyledFragment, Style, StyledFragment,
};

/// The goal of this test is to show how to build sub-documents that you can compose together using
/// spectrum's composition tools.
///
/// ```ts
/// function hello(y) {
///  let x = 1;
///  
///  function world() {
///    console.log("world", x, y);
///  }
/// }
///
/// hello(10)();
/// ```

/// This list of styles is based on
/// https://code.visualstudio.com/api/language-extensions/semantic-highlight-guide
#[allow(unused)]
#[derive(Debug, Copy, Clone)]
enum BasicTokenStyle {
    Namespace,
    Type,
    Class,
    Enum,
    Interface,
    Struct,
    TypeParameter,
    Parameter,
    Variable,
    Property,
    EnumMember,
    Event,
    Function,
    Member,
    Macro,
    Label,
    Comment,
    String,
    Keyword,
    Number,
    Regexp,
    Operator,
}

#[allow(unused)]
#[derive(Debug, Copy, Clone)]
enum TokenModifier {
    Normal,
    Declaration,
    Readonly,
    Static,
    Deprecated,
    Abstract,
    Async,
    Modification,
    Documentation,
    DefaultLibrary,
}

#[derive(Debug, Copy, Clone)]
struct TokenStyle {
    style: BasicTokenStyle,
    modifier: TokenModifier,
}

impl Into<Style> for TokenStyle {
    fn into(self) -> Style {
        Style::default()
    }
}

struct StyledToken<'ctx> {
    style: TokenStyle,
    item: Cow<'ctx, str>,
}

impl<'a> InternedStyledFragment for StyledToken<'a> {
    fn intern(self, intern: &mut Intern) -> StyledFragment {
        let frag = intern.intern(self.item);
        StyledFragment::new(frag, self.style.into())
    }
}

struct Arg<'ctx> {
    ident: StyledToken<'ctx>,
}

impl<'ctx> InternedBoxedDoc for Arg<'ctx> {
    fn intern(self, intern: &mut Intern) -> BoxedDoc {
        let styled = self.ident.intern(intern);
        styled.boxed()
    }
    // fn into(self) -> BoxedDoc {
    //     let styled: spectrum::Styled = self.ident.intern();
    //     styled.boxed()
    // }
}

#[allow(unused)]
struct NamedFunctionDeclaration<'ctx> {
    name: BoxedDoc,
    args: Vec<Arg<'ctx>>,
}

#[test]
fn compose_subdocs() {}
