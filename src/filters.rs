use std::{iter::Peekable, str::Chars};

type StrCursor<'a> = Peekable<Chars<'a>>;

struct HtmlEmitter {
    out: String,
}

impl HtmlEmitter {
    /// Use in contexts where text is expected (anything except the interior attribute
    /// section of a tag or closing tag.)
    fn emit_text_char(&mut self, c: char) {
        match c {
            '&' => self.out.push_str("&amp;"),
            '<' => self.out.push_str("&lt;"),
            '>' => self.out.push_str("&gt;"),
            '"' => self.out.push_str("&quot;"),
            '\'' => self.out.push_str("&#39;"),
            _ => self.out.push(c),
        }
    }

    fn emit_text_str(&mut self, s: &str) {
        for c in s.chars() {
            self.emit_text_char(c);
        }
    }

    /// Lifetime is 'static to ensure that no user input is snuck into this function.
    fn emit_raw_str(&mut self, s: &'static str) {
        self.out.push_str(s);
    }

    /// Escapes a link.
    /// Does *not* sanitize against `javascript:` URIs.
    fn emit_link(&mut self, link: &str, link_text: &str) {
        self.emit_raw_str("<a href=\"");
        // Note: This can't panic because askama's urlencode implementation always returns Ok.
        self.out
            .push_str(&askama::filters::urlencode(link).unwrap());
        self.emit_raw_str("\">");
        self.emit_text_str(link_text);
        self.emit_raw_str("</a>");
    }
}

fn superscript_handler(s: &mut StrCursor, out: &mut HtmlEmitter) {
    s.next();
    out.emit_raw_str("<sup>");
    parse_balanced_parens(s, out);
    // eat closing paren
    s.next();
    out.emit_raw_str("</sup>");
}

fn num_link_handler(s: &mut StrCursor, out: &mut HtmlEmitter) {
    s.next();
    let mut num = String::with_capacity(16);
    for c in s.by_ref() {
        match c {
            '0'..='9' => {
                num.push(c);
            }
            ')' => {
                out.emit_link(&format!("/{num}"), &num);
                return;
            }
            _ => {
                // fallback if not given a number, pretend this wasn't handled at all
                out.emit_text_str(&format!("(#{num}"));
                out.emit_text_char(c);
                return;
            }
        }
    }
}

fn parse_balanced_parens(s: &mut StrCursor, out: &mut HtmlEmitter) {
    let mut brack_count = 0u32;
    while let Some(&c) = s.peek() {
        match c {
            '(' => {
                s.next();
                match s.peek() {
                    Some(&'^') => superscript_handler(s, out),
                    Some(&'#') => num_link_handler(s, out),
                    _ => {
                        brack_count += 1;
                        out.emit_text_char(c);
                    }
                }
            }
            ')' => {
                if brack_count == 0 {
                    return;
                }
                s.next();
                brack_count -= 1;
                out.emit_text_char(c);
            }
            '\\' => {
                s.next();
                match s.next() {
                    None => {
                        out.emit_text_char('\\');
                        return;
                    }
                    Some(d) => out.emit_text_char(d),
                }
            }
            _ => {
                s.next();
                out.emit_text_char(c);
            }
        }
    }
}

fn run_filter(s: &mut StrCursor) -> String {
    let mut res = HtmlEmitter { out: String::new() };
    loop {
        match s.peek() {
            Some(')') => {
                res.emit_text_char(')');
                s.next();
            }
            Some(_) => parse_balanced_parens(s, &mut res),
            None => break,
        }
    }
    res.out
}

pub fn mathfmt<T: std::fmt::Display>(s: T) -> askama::Result<String> {
    Ok(run_filter(&mut s.to_string().chars().peekable()))
}

#[test]
fn mathfmt_test() {
    macro_rules! check {
        ($a:expr, $b:expr) => {
            assert_eq!(mathfmt($a).unwrap(), $b)
        };
    }
    check!(
        r#"<html>&gets' "escaped"</html>"#,
        "&lt;html&gt;&amp;gets&#39; &quot;escaped&quot;&lt;/html&gt;"
    );
    check!("super(^script)", "super<sup>script</sup>");
    check!(
        "super(^script) and after",
        "super<sup>script</sup> and after"
    );
    check!("parens(^bal(an)ce)", "parens<sup>bal(an)ce</sup>");
    check!("unclosed(^is()ignored", "unclosed<sup>is()ignored</sup>");
    check!("not(\\^superscript)", "not(^superscript)");
    check!("backslash escapes \\anything", "backslash escapes anything");
    check!("single\\\\backslash", "single\\backslash");
    check!("no parens means ^ verbatim", "no parens means ^ verbatim");

    check!(
        "links (#537) to numbers",
        "links <a href=\"/537\">537</a> to numbers"
    );

    check!(
        "nested(^link(#11) with text after)",
        "nested<sup>link<a href=\"/11\">11</a> with text after</sup>"
    );

    check!(
        "nested(^link(#11))",
        "nested<sup>link<a href=\"/11\">11</a></sup>"
    );

    check!(
        "links fail without parens #11",
        "links fail without parens #11"
    );
}
