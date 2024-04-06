use std::{iter::Peekable, str::Chars};

type StrCursor<'a> = Peekable<Chars<'a>>;

fn parse_balanced_parens(s: &mut StrCursor, out: &mut String) {
    let mut brack_count = 0u32;
    while let Some(c) = s.next() {
        match c {
            '(' => {
                brack_count += 1;
                out.push(c);
            },
            ')' => {
                if brack_count == 0 {
                    return;
                }
                brack_count -= 1;
                out.push(c);
            }
            '\\' => match s.next() {
                None => {
                    out.push('\\');
                    return;
                }
                Some(d) => out.push(d),
            },
            '^' if s.peek() == Some(&'(') => {
                s.next();
                out.push_str("<sup>");
                parse_balanced_parens(s, out);
                out.push_str("</sup>");
            }
            _ => out.push(c),
        }
    }
}

fn parse(s: &mut StrCursor) -> String {
    let mut res = String::new();
    while s.peek().is_some() {
        parse_balanced_parens(s, &mut res);
    }
    res
}

pub fn mathfmt<T: std::fmt::Display>(s: T) -> askama::Result<String> {
    let s = askama::filters::escape(askama_escape::Html, s)?.to_string();
    Ok(parse(&mut s.chars().peekable()))
}

#[test]
fn mathfmt_test() {
    macro_rules! check {
        ($a:expr, $b:expr) => {
            assert_eq!(mathfmt($a).unwrap(), $b)
        };
    }
    check!(
        "<html>gets escaped</html>",
        "&lt;html&gt;gets escaped&lt;/html&gt;"
    );
    check!("super^(script)", "super<sup>script</sup>");
    check!("parens^(bal(an)ce)", "parens<sup>bal(an)ce</sup>");
    check!("unclosed^(is()ignored", "unclosed<sup>is()ignored</sup>");
    check!("not\\^(superscript)", "not^(superscript)");
    check!("backslash escapes \\anything", "backslash escapes anything");
    check!("single\\\\backslash", "single\\backslash");
    check!("no parens means ^ verbatim", "no parens means ^ verbatim");
}
