use crate::theme::ast::{AttrValue, Node, Template};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub message: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ParseError {}

pub fn parse_template(source: &str) -> Result<Template, ParseError> {
    if source.len() > crate::theme::security::MAX_THEME_FILE_BYTES {
        return Err(ParseError {
            message: "template source is too large".into(),
        });
    }
    let mut parser = Parser {
        input: source,
        pos: 0,
        depth: 0,
    };
    let nodes = parser.parse_nodes_until(&[])?;
    Ok(Template { nodes })
}

struct Parser<'a> {
    input: &'a str,
    pos: usize,
    depth: usize,
}

impl<'a> Parser<'a> {
    fn remaining(&self) -> &'a str {
        &self.input[self.pos..]
    }

    fn starts_with(&self, s: &str) -> bool {
        self.remaining().starts_with(s)
    }

    fn starts_with_stop(&self, stop: &str) -> bool {
        if stop.starts_with("</") {
            self.remaining()
                .get(..stop.len())
                .is_some_and(|prefix| prefix.eq_ignore_ascii_case(stop))
        } else if stop.starts_with('@') {
            self.starts_with_directive(stop)
        } else {
            self.starts_with(stop)
        }
    }

    fn starts_with_directive(&self, name: &str) -> bool {
        let Some(rest) = self.remaining().strip_prefix(name) else {
            return false;
        };
        rest.chars()
            .next()
            .is_none_or(|next| next.is_whitespace() || next == '(')
    }

    fn advance(&mut self, n: usize) {
        self.pos = (self.pos + n).min(self.input.len());
    }

    fn peek_char(&self) -> Option<char> {
        self.remaining().chars().next()
    }

    fn parse_nodes_until(&mut self, stop: &[&str]) -> Result<Vec<Node>, ParseError> {
        let mut nodes = Vec::new();
        while !self.remaining().is_empty() {
            if stop.iter().any(|s| self.starts_with_stop(s)) {
                break;
            }
            nodes.push(self.parse_node(stop)?);
        }
        Ok(nodes)
    }

    fn parse_node(&mut self, stop: &[&str]) -> Result<Node, ParseError> {
        if self.depth >= crate::theme::security::MAX_PARSE_DEPTH {
            return Err(ParseError {
                message: format!("template nesting exceeds {} levels", crate::theme::security::MAX_PARSE_DEPTH),
            });
        }
        self.depth += 1;
        let result = self.parse_node_inner(stop);
        self.depth -= 1;
        result
    }

    fn parse_node_inner(&mut self, stop: &[&str]) -> Result<Node, ParseError> {
        let rem = self.remaining();
        if rem.starts_with("{{") {
            return self.parse_interp(false);
        }
        if rem.starts_with("{!!") {
            return self.parse_interp(true);
        }
        if rem.starts_with('@') {
            return self.parse_directive(stop);
        }
        if rem.starts_with("<!--") {
            return self.parse_comment();
        }
        if rem.starts_with('<') {
            return self.parse_element(stop);
        }
        self.parse_text(stop)
    }

    fn parse_text(&mut self, stop: &[&str]) -> Result<Node, ParseError> {
        let start = self.pos;
        while let Some(ch) = self.peek_char() {
            if self.starts_with("{{")
                || self.starts_with("{!!")
                || self.starts_with("@")
                || self.starts_with("<")
                || stop.iter().any(|s| self.starts_with_stop(s))
            {
                break;
            }
            self.advance(ch.len_utf8());
        }
        Ok(Node::Text(self.input[start..self.pos].to_string()))
    }

    fn parse_comment(&mut self) -> Result<Node, ParseError> {
        self.advance(4);
        if let Some(end) = self.remaining().find("-->") {
            self.advance(end + 3);
        } else {
            return Err(ParseError {
                message: format!("unclosed comment at byte {}", self.pos.saturating_sub(4)),
            });
        }
        Ok(Node::Text(String::new()))
    }

    fn parse_interp(&mut self, raw: bool) -> Result<Node, ParseError> {
        let (open, close) = if raw { ("{!!", "!!}") } else { ("{{", "}}") };
        if !self.starts_with(open) {
            return Err(ParseError {
                message: "expected interpolation".into(),
            });
        }
        self.advance(open.len());
        let Some(end) = self.remaining().find(close) else {
            return Err(ParseError {
                message: "unclosed interpolation".into(),
            });
        };
        let expr = self.remaining()[..end].trim().to_string();
        self.advance(end + close.len());
        Ok(Node::Interp { expr, raw })
    }

    fn parse_directive(&mut self, _stop: &[&str]) -> Result<Node, ParseError> {
        if self.starts_with_directive("@if") {
            return self.parse_if();
        }
        if self.starts_with_directive("@foreach") {
            return self.parse_foreach();
        }
        if self.starts_with_directive("@include") {
            return self.parse_include();
        }
        // Treat unknown @ as text so CSS/emails aren't destroyed
        let start = self.pos;
        self.advance(1);
        while let Some(ch) = self.peek_char() {
            if ch.is_whitespace() || ch == '<' || ch == '{' || ch == '@' {
                break;
            }
            self.advance(ch.len_utf8());
        }
        Ok(Node::Text(self.input[start..self.pos].to_string()))
    }

    fn parse_if(&mut self) -> Result<Node, ParseError> {
        let mut arms = Vec::new();
        // @if (cond)
        let cond = self.take_directive_condition("@if")?;
        let body = self.parse_nodes_until(&["@elseif", "@else", "@endif"])?;
        arms.push((cond, body));

        while self.starts_with_directive("@elseif") {
            let cond = self.take_directive_condition("@elseif")?;
            let body = self.parse_nodes_until(&["@elseif", "@else", "@endif"])?;
            arms.push((cond, body));
        }

        let else_body = if self.starts_with_directive("@else") {
            self.advance("@else".len());
            self.skip_ws_newline();
            self.parse_nodes_until(&["@endif"])?
        } else {
            Vec::new()
        };

        if !self.starts_with_directive("@endif") {
            return Err(ParseError {
                message: "expected @endif".into(),
            });
        }
        self.advance("@endif".len());
        Ok(Node::If { arms, else_body })
    }

    fn parse_foreach(&mut self) -> Result<Node, ParseError> {
        if !self.starts_with_directive("@foreach") {
            return Err(ParseError {
                message: "expected @foreach".into(),
            });
        }
        self.advance("@foreach".len());
        self.skip_ws();
        if !self.starts_with("(") {
            return Err(ParseError {
                message: "expected ( after @foreach".into(),
            });
        }
        self.advance(1);
        let Some(end) = self.find_matching_paren() else {
            return Err(ParseError {
                message: "unclosed @foreach (".into(),
            });
        };
        let inner = self.remaining()[..end].trim().to_string();
        self.advance(end + 1);
        self.skip_ws_newline();

        // "items as item" or "items as item, index"
        let (collection, item) = if let Some((left, right)) = inner.split_once(" as ") {
            (
                left.trim().to_string(),
                right.split(',').next().unwrap().trim().to_string(),
            )
        } else {
            return Err(ParseError {
                message: "expected @foreach (collection as item)".into(),
            });
        };

        let body = self.parse_nodes_until(&["@endforeach"])?;
        if !self.starts_with_directive("@endforeach") {
            return Err(ParseError {
                message: "expected @endforeach".into(),
            });
        }
        self.advance("@endforeach".len());
        Ok(Node::ForEach {
            collection,
            item,
            body,
        })
    }

    fn parse_include(&mut self) -> Result<Node, ParseError> {
        self.advance("@include".len());
        self.skip_ws();
        if !self.starts_with("(") {
            return Err(ParseError {
                message: "expected ( after @include".into(),
            });
        }
        self.advance(1);
        self.skip_ws();
        let quote = self.peek_char().unwrap_or('"');
        if quote != '\'' && quote != '"' {
            return Err(ParseError {
                message: "expected quoted include path".into(),
            });
        }
        self.advance(1);
        let Some(end) = self.remaining().find(quote) else {
            return Err(ParseError {
                message: "unclosed include path".into(),
            });
        };
        let path = self.remaining()[..end].to_string();
        self.advance(end + 1);
        self.skip_ws();
        if self.starts_with(")") {
            self.advance(1);
        }
        Ok(Node::Include { path })
    }

    fn take_directive_condition(&mut self, name: &str) -> Result<String, ParseError> {
        if !self.starts_with_directive(name) {
            return Err(ParseError {
                message: format!("expected {name}"),
            });
        }
        self.advance(name.len());
        self.skip_ws();
        if !self.starts_with("(") {
            return Err(ParseError {
                message: format!("expected ( after {name}"),
            });
        }
        self.advance(1);
        let Some(end) = self.find_matching_paren() else {
            return Err(ParseError {
                message: format!("unclosed {name} condition"),
            });
        };
        let cond = self.remaining()[..end].trim().to_string();
        self.advance(end + 1);
        self.skip_ws_newline();
        Ok(cond)
    }

    fn find_matching_paren(&self) -> Option<usize> {
        let bytes = self.remaining().as_bytes();
        let mut depth = 1usize;
        let mut in_single = false;
        let mut in_double = false;
        let mut i = 0usize;
        while i < bytes.len() {
            let byte = bytes[i];
            if in_single {
                if byte == b'\'' {
                    in_single = false;
                }
                i += 1;
                continue;
            }
            if in_double {
                if byte == b'"' {
                    in_double = false;
                }
                i += 1;
                continue;
            }
            match byte {
                b'\'' => in_single = true,
                b'"' => in_double = true,
                b'(' => depth += 1,
                b')' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(i);
                    }
                }
                _ => {}
            }
            i += 1;
        }
        None
    }

    fn parse_element(&mut self, stop: &[&str]) -> Result<Node, ParseError> {
        if self.starts_with("</") {
            let closing = self
                .remaining()
                .split_once('>')
                .map(|(tag, _)| tag)
                .unwrap_or(self.remaining());
            return Err(ParseError {
                message: format!("unexpected closing tag `{closing}>` at byte {}", self.pos),
            });
        }
        self.advance(1); // <
        let tag = self.read_ident().to_ascii_lowercase();
        if tag.is_empty() {
            return Ok(Node::Text("<".into()));
        }
        self.skip_ws();
        let mut attrs = Vec::new();
        while let Some(ch) = self.peek_char() {
            if ch == '>' || ch == '/' {
                break;
            }
            let name = self.read_attr_name().to_ascii_lowercase();
            if name.is_empty() {
                break;
            }
            self.skip_ws();
            let value = if self.starts_with("=") {
                self.advance(1);
                self.skip_ws();
                self.read_attr_value()?
            } else {
                AttrValue::Literal(String::new())
            };
            attrs.push((name, value));
            self.skip_ws();
        }

        let void = is_void(&tag);
        let self_closing = self.starts_with("/>") || (self.starts_with(">") && void);
        if self.starts_with("/>") {
            self.advance(2);
            return Ok(Node::Element {
                tag,
                attrs,
                children: Vec::new(),
                self_closing: true,
            });
        }
        if self.starts_with(">") {
            self.advance(1);
        }
        if void || self_closing {
            return Ok(Node::Element {
                tag,
                attrs,
                children: Vec::new(),
                self_closing: true,
            });
        }

        let close = format!("</{tag}>");
        let mut child_stops = Vec::with_capacity(stop.len() + 1);
        child_stops.push(close.as_str());
        child_stops.extend_from_slice(stop);
        let children = self.parse_nodes_until(&child_stops)?;
        if !self.starts_with_stop(&close) {
            return Err(ParseError {
                message: format!("missing closing tag `{close}` at byte {}", self.pos),
            });
        }
        self.advance(close.len());
        Ok(Node::Element {
            tag,
            attrs,
            children,
            self_closing: false,
        })
    }

    fn read_ident(&mut self) -> String {
        let start = self.pos;
        while let Some(ch) = self.peek_char() {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' || ch == ':' {
                self.advance(ch.len_utf8());
            } else {
                break;
            }
        }
        self.input[start..self.pos].to_string()
    }

    fn read_attr_name(&mut self) -> String {
        self.read_ident()
    }

    fn read_attr_value(&mut self) -> Result<AttrValue, ParseError> {
        let rem = self.remaining();
        if rem.starts_with("{{") {
            let Node::Interp { expr, raw } = self.parse_interp(false)? else {
                unreachable!()
            };
            return Ok(AttrValue::Interp { expr, raw });
        }
        if rem.starts_with("{!!") {
            let Node::Interp { expr, raw } = self.parse_interp(true)? else {
                unreachable!()
            };
            return Ok(AttrValue::Interp { expr, raw });
        }
        let quote = self.peek_char();
        if quote == Some('"') || quote == Some('\'') {
            let q = quote.unwrap();
            self.advance(1);
            let start = self.pos;
            while let Some(ch) = self.peek_char() {
                if ch == q {
                    break;
                }
                self.advance(ch.len_utf8());
            }
            let lit = self.input[start..self.pos].to_string();
            if self.peek_char() == Some(q) {
                self.advance(1);
            }
            // Support "{{ x }}" inside quoted attrs
            if lit.contains("{{") {
                let trimmed = lit.trim();
                if let Some(inner) = trimmed
                    .strip_prefix("{{")
                    .and_then(|s| s.strip_suffix("}}"))
                {
                    return Ok(AttrValue::Interp {
                        expr: inner.trim().to_string(),
                        raw: false,
                    });
                }
            }
            return Ok(AttrValue::Literal(lit));
        }
        let start = self.pos;
        while let Some(ch) = self.peek_char() {
            if ch.is_whitespace() || ch == '>' || ch == '/' {
                break;
            }
            self.advance(ch.len_utf8());
        }
        Ok(AttrValue::Literal(self.input[start..self.pos].to_string()))
    }

    fn skip_ws(&mut self) {
        while let Some(c) = self.peek_char() {
            if !c.is_whitespace() {
                break;
            }
            self.advance(c.len_utf8());
        }
    }

    fn skip_ws_newline(&mut self) {
        self.skip_ws();
    }
}

fn is_void(tag: &str) -> bool {
    matches!(
        tag,
        "area"
            | "base"
            | "br"
            | "col"
            | "embed"
            | "hr"
            | "img"
            | "input"
            | "link"
            | "meta"
            | "param"
            | "source"
            | "track"
            | "wbr"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_forum_index_template() {
        let src = include_str!("../../themes/default/forum/index.html");
        let template = parse_template(src).expect("forum index should parse");
        assert!(!template.nodes.is_empty());
    }

    #[test]
    fn parses_forum_thread_template() {
        let src = include_str!("../../themes/default/forum/thread.html");
        let template = parse_template(src).expect("forum thread should parse");
        assert!(!template.nodes.is_empty());
    }

    #[test]
    fn parses_every_default_theme_template() {
        for entry in crate::theme::loader::embedded_pack_entries()
            .into_iter()
            .filter(|entry| entry.language == "HTML")
        {
            parse_template(&entry.content)
                .unwrap_or_else(|error| panic!("{} failed to parse: {error}", entry.path));
        }
    }

    #[test]
    fn parses_foreach_and_if() {
        let src = r#"
@if (boards_count)
  @foreach (boards as board)
    <a href="{{ board.href }}">{{ board.name }}</a>
  @endforeach
@else
  <p>empty</p>
@endif
"#;
        parse_template(src).expect("control flow should parse");
    }

    #[test]
    fn rejects_mismatched_and_missing_closing_tags() {
        let mismatched = parse_template("<div><span>hello</div>");
        assert!(mismatched.is_err());
        let mismatched_message = mismatched.unwrap_err().message;
        assert!(
            mismatched_message.contains("missing closing tag")
                || mismatched_message.contains("unexpected closing tag"),
            "unexpected message: {mismatched_message}"
        );

        let missing = parse_template("<section><p>hello</p>");
        assert!(missing.is_err());
        assert!(missing.unwrap_err().message.contains("missing closing tag"));
    }

    #[test]
    fn handles_case_insensitive_tags_and_unicode_whitespace() {
        parse_template("<DIV><SPAN>hello</span></div>")
            .expect("HTML tags should be case insensitive");
        parse_template("@if\u{2003}(enabled)<p>yes</p>@endif")
            .expect("unicode whitespace must advance on UTF-8 boundaries");
    }

    #[test]
    fn rejects_unclosed_comments() {
        let error = parse_template("before <!-- never closed").unwrap_err();
        assert!(error.message.contains("unclosed comment"));
    }

    #[test]
    fn parses_nested_config_conditions() {
        let template = parse_template(
            r#"@if (config("hero_title"))
<p>{{ config("hero_title") }}</p>
@endif"#,
        )
        .expect("nested parentheses in @if should parse");
        assert!(!template.nodes.is_empty());
    }

    #[test]
    fn does_not_swallow_endif_inside_unclosed_element() {
        let error = parse_template(
            r#"@if (enabled)
<div>
  <p>hello</p>
@endif
</div>"#,
        )
        .unwrap_err();
        assert!(error.message.contains("missing closing tag"));
    }

    #[test]
    fn rejects_extreme_nesting() {
        let mut src = String::new();
        for _ in 0..80 {
            src.push_str("<div>");
        }
        src.push_str("x");
        for _ in 0..80 {
            src.push_str("</div>");
        }
        let err = parse_template(&src).unwrap_err();
        assert!(err.message.contains("nesting exceeds"));
    }
}
