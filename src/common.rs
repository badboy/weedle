use Parse;
use term;
use literal::*;

impl<T: Parse> Parse for Option<T> {
    named!(parse -> Self, opt!(weedle!(T)));
}

impl<T: Parse> Parse for Box<T> {
    named!(parse -> Self, do_parse!(
        inner: weedle!(T) >>
        (Box::new(inner))
    ));
}

/// Parses `item1 item2 item3...`
impl<T: Parse> Parse for Vec<T> {
    named!(parse -> Self, many0!(weedle!(T)));
}

impl<T: Parse, U: Parse> Parse for (T, U) {
    named!(parse-> Self, do_parse!(
        f: weedle!(T) >>
        s: weedle!(U) >>
        ((f, s))
    ));
}

impl<T: Parse, U: Parse, V: Parse> Parse for (T, U, V) {
    named!(parse-> Self, do_parse!(
        f: weedle!(T) >>
        s: weedle!(U) >>
        t: weedle!(V) >>
        ((f, s, t))
    ));
}

/// Parses `{ body }`
#[derive(Debug, PartialEq, Clone)]
pub struct Parenthesized<T> {
    pub open_paren: term::OpenParen,
    pub body: T,
    pub close_paren: term::CloseParen,
}

impl<T: Parse> Parse for Parenthesized<T> {
    named!(parse -> Self, do_parse!(
        open_paren: weedle!(term::OpenParen) >>
        body: weedle!(T) >>
        close_paren: weedle!(term::CloseParen) >>
        (Parenthesized {  open_paren, body, close_paren })
    ));
}

/// Parses `[ body ]`
#[derive(Debug, PartialEq, Clone)]
pub struct Bracketed<T> {
    pub open_bracket: term::OpenBracket,
    pub body: T,
    pub close_bracket: term::CloseBracket,
}

impl<T: Parse> Parse for Bracketed<T> {
    named!(parse -> Self, do_parse!(
        open_bracket: weedle!(term::OpenBracket) >>
        body: weedle!(T) >>
        close_bracket: weedle!(term::CloseBracket) >>
        (Bracketed { open_bracket, body, close_bracket })
    ));
}

/// Parses `( body )`
#[derive(Debug, PartialEq, Clone)]
pub struct Braced<T> {
    pub open_brace: term::OpenBrace,
    pub body: T,
    pub close_brace: term::CloseBrace,
}

impl<T: Parse> Parse for Braced<T> {
    named!(parse -> Self, do_parse!(
        open_brace: weedle!(term::OpenBrace) >>
        body: weedle!(T) >>
        close_brace: weedle!(term::CloseBrace) >>
        (Braced { open_brace, body, close_brace })
    ));
}

/// Parses `< body >`
#[derive(Debug, PartialEq, Clone)]
pub struct Generics<T> {
    pub open_angle: term::LessThan,
    pub body: T,
    pub close_angle: term::GreaterThan
}

impl<T: Parse> Parse for Generics<T> {
    named!(parse -> Self, do_parse!(
        open_angle: weedle!(term::LessThan) >>
        body: weedle!(T) >>
        close_angle: weedle!(term::GreaterThan) >>
        (Generics { open_angle, body, close_angle })
    ));
}

/// Parses `(item1, item2, item3,...)?`
#[derive(Debug, PartialEq, Clone)]
pub struct Punctuated<T, S> {
    pub list: Vec<T>,
    pub separator: S,
}

impl<T: Parse, S: Parse + ::std::default::Default> Parse for Punctuated<T, S> {
    named!(parse -> Self, do_parse!(
        list: separated_list!(weedle!(S), weedle!(T)) >>
        (Punctuated { list, separator: S::default() })
    ));
}

/// Parses `item1, item2, item3, ...`
#[derive(Debug, PartialEq, Clone)]
pub struct PunctuatedNonEmpty<T, S> {
    pub list: Vec<T>,
    pub separator: S
}

impl<T: Parse, S: Parse + ::std::default::Default> Parse for PunctuatedNonEmpty<T, S> {
    named!(parse -> Self, do_parse!(
        list: separated_nonempty_list!(weedle!(S), weedle!(T)) >>
        (PunctuatedNonEmpty { list, separator: S::default() })
    ));
}

/// Represents an identifier
///
/// Follows `/_?[A-Za-z][0-9A-Z_a-z-]*/`
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Identifier {
    pub name: String
}

impl Parse for Identifier {
    named!(parse -> Self, do_parse!(
        name: ws!(re_capture_static!(r"^(_?[A-Za-z][0-9A-Z_a-z-]*)")) >>
        (Identifier { name: name[0].to_string() })
    ));
}

/// Parses rhs of an assignment expression. Ex: `= 45`
#[derive(Debug, PartialEq, Clone)]
pub struct Default {
    pub assign: term!(=),
    pub value: DefaultValue,
}

impl Parse for Default {
    named!(parse -> Self, do_parse!(
        assign: weedle!(term!(=)) >>
        value: weedle!(DefaultValue) >>
        (Default { assign, value })
    ));
}

#[cfg(test)]
mod test {
    use super::*;

    test!(should_parse_optional_present { "one" =>
        "";
        Option<Identifier>;
        is_some();
    });

    test!(should_parse_optional_not_present { "" =>
        "";
        Option<Identifier>;
        is_none();
    });

    test!(should_parse_boxed { "one" =>
        "";
        Box<Identifier>;
    });

    test!(should_parse_vec { "one two three" =>
        "";
        Vec<Identifier>;
        len() == 3;
    });

    test!(should_parse_parenthesized { "{ one }" =>
        "";
        Parenthesized<Identifier>;
        body.name == "one";
    });

    test!(should_parse_bracketed { "[ one ]" =>
        "";
        Bracketed<Identifier>;
        body.name == "one";
    });

    test!(should_parse_braced { "( one )" =>
        "";
        Braced<Identifier>;
        body.name == "one";
    });

    test!(should_parse_generics { "<one>" =>
        "";
        Generics<Identifier>;
        body.name == "one";
    });

    test!(should_parse_generics_two { "<one, two>" =>
        "";
        Generics<(Identifier, term!(,), Identifier)>;
        body.0.name == "one";
        body.2.name == "two";
    });

    test!(should_parse_comma_separated_values { "one, two, three" =>
        "";
        Punctuated<Identifier, term!(,)>;
        list.len() == 3;
    });

    test!(err should_not_parse_comma_separated_values_empty { "" =>
        PunctuatedNonEmpty<Identifier, term!(,)>
    });

    test!(should_parse_identifier { "hello" =>
        "";
        Identifier;
        name == "hello";
    });

    test!(should_parse_numbered_identifier { "hello5" =>
        "";
        Identifier;
        name == "hello5";
    });

    test!(should_parse_underscored_identifier { "_hello_" =>
        "";
        Identifier;
        name == "_hello_";
    });

    test!(should_parse_identifier_surrounding_with_spaces { "  hello  " =>
        "";
        Identifier;
        name == "hello";
    });

    test!(should_parse_identifier_preceeding_others { "hello  note" =>
        "note";
        Identifier;
        name == "hello";
    });

    test!(should_parse_identifier_attached_to_symbol { "hello=" =>
        "=";
        Identifier;
        name == "hello";
    });
}
