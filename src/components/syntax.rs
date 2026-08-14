#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Plain,
    Comment,
    String,
    Keyword,
    Property,
    Number,
    Tag,
    Attribute,
    Punctuation,
}

impl TokenKind {
    pub const fn class(self) -> &'static str {
        match self {
            Self::Plain => "tok-plain",
            Self::Comment => "tok-comment",
            Self::String => "tok-string",
            Self::Keyword => "tok-keyword",
            Self::Property => "tok-property",
            Self::Number => "tok-number",
            Self::Tag => "tok-tag",
            Self::Attribute => "tok-attr",
            Self::Punctuation => "tok-punct",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub kind: TokenKind,
    pub start: u32,
    pub end: u32,
}

pub fn highlight_spans(source: &str, language: &str) -> Vec<Span> {
    match language {
        "HTML" | "SVG" => highlight_spot_html(source),
        "CSS" => highlight_css(source),
        "JS" | "TS" | "JSON" => highlight_js_like(source),
        _ => {
            if source.is_empty() {
                Vec::new()
            } else {
                vec![Span {
                    kind: TokenKind::Plain,
                    start: 0,
                    end: source.len() as u32,
                }]
            }
        }
    }
}

pub fn highlighted_html(source: &str, language: &str) -> String {
    let spans = highlight_spans(source, language);
    let mut out = String::with_capacity(source.len().saturating_add(spans.len().saturating_mul(28)));
    for span in spans {
        let start = span.start as usize;
        let end = (span.end as usize).min(source.len());
        if start >= end {
            continue;
        }
        out.push_str("<span class=\"");
        out.push_str(span.kind.class());
        out.push_str("\">");
        push_escaped(&mut out, &source[start..end]);
        out.push_str("</span>");
    }
    out
}

fn push_escaped(out: &mut String, text: &str) {
    for ch in text.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(ch),
        }
    }
}

fn push_span(out: &mut Vec<Span>, kind: TokenKind, start: usize, end: usize) {
    if start < end {
        out.push(Span {
            kind,
            start: start as u32,
            end: end as u32,
        });
    }
}

fn highlight_css(source: &str) -> Vec<Span> {
    let bytes = source.as_bytes();
    let mut out = Vec::with_capacity(bytes.len() / 8 + 8);
    let mut i = 0;
    let mut plain = 0usize;
    let mut in_block = false;

    let flush_plain = |out: &mut Vec<Span>, plain: &mut usize, i: usize| {
        if *plain < i {
            push_span(out, TokenKind::Plain, *plain, i);
            *plain = i;
        }
    };

    while i < bytes.len() {
        if bytes[i] == b'/' && bytes.get(i + 1) == Some(&b'*') {
            flush_plain(&mut out, &mut plain, i);
            let start = i;
            i += 2;
            while i + 1 < bytes.len() && !(bytes[i] == b'*' && bytes[i + 1] == b'/') {
                i += 1;
            }
            i = (i + 2).min(bytes.len());
            push_span(&mut out, TokenKind::Comment, start, i);
            plain = i;
            continue;
        }

        if bytes[i] == b'"' || bytes[i] == b'\'' {
            flush_plain(&mut out, &mut plain, i);
            let quote = bytes[i];
            let start = i;
            i += 1;
            while i < bytes.len() {
                if bytes[i] == b'\\' {
                    i = (i + 2).min(bytes.len());
                    continue;
                }
                if bytes[i] == quote {
                    i += 1;
                    break;
                }
                i += 1;
            }
            push_span(&mut out, TokenKind::String, start, i);
            plain = i;
            continue;
        }

        if bytes[i] == b'@' {
            flush_plain(&mut out, &mut plain, i);
            let start = i;
            i += 1;
            while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'-') {
                i += 1;
            }
            push_span(&mut out, TokenKind::Keyword, start, i);
            plain = i;
            continue;
        }

        if bytes[i].is_ascii_digit()
            || (bytes[i] == b'#' && bytes.get(i + 1).is_some_and(|b| b.is_ascii_hexdigit()))
        {
            flush_plain(&mut out, &mut plain, i);
            let start = i;
            if bytes[i] == b'#' {
                i += 1;
                while i < bytes.len() && bytes[i].is_ascii_hexdigit() {
                    i += 1;
                }
            } else {
                while i < bytes.len()
                    && (bytes[i].is_ascii_digit() || bytes[i] == b'.' || bytes[i] == b'%')
                {
                    i += 1;
                }
            }
            push_span(&mut out, TokenKind::Number, start, i);
            plain = i;
            continue;
        }

        if bytes[i].is_ascii_alphabetic() || bytes[i] == b'_' || bytes[i] == b'-' {
            let start = i;
            i += 1;
            while i < bytes.len()
                && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'-' || bytes[i] == b'_')
            {
                i += 1;
            }
            let word = &source[start..i];
            let mut j = i;
            while j < bytes.len() && bytes[j].is_ascii_whitespace() {
                j += 1;
            }
            let kind = if in_block && bytes.get(j) == Some(&b':') {
                Some(TokenKind::Property)
            } else if matches!(
                word,
                "important" | "from" | "to" | "and" | "or" | "not" | "only" | "screen" | "print"
            ) {
                Some(TokenKind::Keyword)
            } else {
                None
            };
            if let Some(kind) = kind {
                flush_plain(&mut out, &mut plain, start);
                push_span(&mut out, kind, start, i);
                plain = i;
            }
            continue;
        }

        if bytes[i] == b'{' {
            flush_plain(&mut out, &mut plain, i);
            in_block = true;
            push_span(&mut out, TokenKind::Punctuation, i, i + 1);
            i += 1;
            plain = i;
            continue;
        }
        if bytes[i] == b'}' {
            flush_plain(&mut out, &mut plain, i);
            in_block = false;
            push_span(&mut out, TokenKind::Punctuation, i, i + 1);
            i += 1;
            plain = i;
            continue;
        }
        if matches!(bytes[i], b':' | b';' | b',' | b'(' | b')' | b'[' | b']') {
            flush_plain(&mut out, &mut plain, i);
            push_span(&mut out, TokenKind::Punctuation, i, i + 1);
            i += 1;
            plain = i;
            continue;
        }

        i += source[i..].chars().next().map(|c| c.len_utf8()).unwrap_or(1);
    }

    flush_plain(&mut out, &mut plain, i);
    out
}

fn highlight_spot_html(source: &str) -> Vec<Span> {
    const DIRECTIVES: &[&[u8]] = &[
        b"@endforeach",
        b"@foreach",
        b"@elseif",
        b"@endif",
        b"@include",
        b"@else",
        b"@if",
    ];

    let bytes = source.as_bytes();
    let mut out = Vec::with_capacity(bytes.len() / 6 + 8);
    let mut i = 0;
    let mut plain = 0usize;

    let flush_plain = |out: &mut Vec<Span>, plain: &mut usize, i: usize| {
        if *plain < i {
            push_span(out, TokenKind::Plain, *plain, i);
            *plain = i;
        }
    };

    while i < bytes.len() {
        if bytes[i..].starts_with(b"<!--") {
            flush_plain(&mut out, &mut plain, i);
            let start = i;
            i += 4;
            while i + 2 < bytes.len() && &bytes[i..i + 3] != b"-->" {
                i += 1;
            }
            i = (i + 3).min(bytes.len());
            push_span(&mut out, TokenKind::Comment, start, i);
            plain = i;
            continue;
        }

        if bytes[i..].starts_with(b"{!!") {
            flush_plain(&mut out, &mut plain, i);
            push_span(&mut out, TokenKind::Punctuation, i, i + 3);
            i += 3;
            let expr_start = i;
            while i + 2 < bytes.len() && &bytes[i..i + 3] != b"!!}" {
                i += 1;
            }
            push_span(&mut out, TokenKind::Property, expr_start, i);
            if i + 2 < bytes.len() {
                push_span(&mut out, TokenKind::Punctuation, i, i + 3);
                i += 3;
            }
            plain = i;
            continue;
        }

        if bytes[i..].starts_with(b"{{") {
            flush_plain(&mut out, &mut plain, i);
            push_span(&mut out, TokenKind::Punctuation, i, i + 2);
            i += 2;
            let expr_start = i;
            while i + 1 < bytes.len() && !(bytes[i] == b'}' && bytes[i + 1] == b'}') {
                i += 1;
            }
            push_span(&mut out, TokenKind::Property, expr_start, i);
            if i + 1 < bytes.len() {
                push_span(&mut out, TokenKind::Punctuation, i, i + 2);
                i += 2;
            }
            plain = i;
            continue;
        }

        if bytes[i] == b'@' {
            let mut matched: Option<&[u8]> = None;
            for dir in DIRECTIVES {
                if bytes[i..].starts_with(dir) {
                    let after = i + dir.len();
                    let boundary_ok = after >= bytes.len()
                        || (!bytes[after].is_ascii_alphanumeric() && bytes[after] != b'_');
                    if boundary_ok {
                        matched = Some(dir);
                        break;
                    }
                }
            }
            if let Some(dir) = matched {
                flush_plain(&mut out, &mut plain, i);
                let start = i;
                i += dir.len();
                push_span(&mut out, TokenKind::Keyword, start, i);

                let mut j = i;
                while j < bytes.len() && bytes[j].is_ascii_whitespace() {
                    j += 1;
                }
                if j < bytes.len() && bytes[j] == b'(' {
                    if j > i {
                        push_span(&mut out, TokenKind::Plain, i, j);
                    }
                    highlight_directive_args(bytes, &mut out, j);
                    i = skip_balanced_parens(bytes, j);
                }
                plain = i;
                continue;
            }
        }

        if bytes[i] == b'<' {
            flush_plain(&mut out, &mut plain, i);
            highlight_html_tag(bytes, &mut out, &mut i);
            plain = i;
            continue;
        }

        i += source[i..].chars().next().map(|c| c.len_utf8()).unwrap_or(1);
    }

    flush_plain(&mut out, &mut plain, i);
    out
}

fn skip_balanced_parens(bytes: &[u8], open_at: usize) -> usize {
    if open_at >= bytes.len() || bytes[open_at] != b'(' {
        return open_at;
    }
    let mut i = open_at + 1;
    let mut depth = 1i32;
    while i < bytes.len() && depth > 0 {
        let ch = bytes[i];
        if ch == b'"' || ch == b'\'' {
            let quote = ch;
            i += 1;
            while i < bytes.len() && bytes[i] != quote {
                if bytes[i] == b'\\' {
                    i = (i + 2).min(bytes.len());
                    continue;
                }
                i += 1;
            }
            if i < bytes.len() {
                i += 1;
            }
            continue;
        }
        if ch == b'(' {
            depth += 1;
        } else if ch == b')' {
            depth -= 1;
        }
        i += 1;
    }
    i
}

fn highlight_directive_args(bytes: &[u8], out: &mut Vec<Span>, open_at: usize) {
    if open_at >= bytes.len() || bytes[open_at] != b'(' {
        return;
    }
    push_span(out, TokenKind::Punctuation, open_at, open_at + 1);
    let mut i = open_at + 1;
    let mut depth = 1i32;
    let mut seg = i;
    while i < bytes.len() && depth > 0 {
        let ch = bytes[i];
        if ch == b'"' || ch == b'\'' {
            if seg < i {
                highlight_directive_expr(out, bytes, seg, i);
            }
            let quote = ch;
            let s = i;
            i += 1;
            while i < bytes.len() && bytes[i] != quote {
                if bytes[i] == b'\\' {
                    i = (i + 2).min(bytes.len());
                    continue;
                }
                i += 1;
            }
            if i < bytes.len() {
                i += 1;
            }
            push_span(out, TokenKind::String, s, i);
            seg = i;
            continue;
        }
        if ch == b'(' {
            depth += 1;
            i += 1;
            continue;
        }
        if ch == b')' {
            depth -= 1;
            if depth == 0 {
                if seg < i {
                    highlight_directive_expr(out, bytes, seg, i);
                }
                push_span(out, TokenKind::Punctuation, i, i + 1);
                return;
            }
            i += 1;
            continue;
        }
        i += 1;
    }
    if seg < i {
        highlight_directive_expr(out, bytes, seg, i);
    }
}

fn highlight_directive_expr(out: &mut Vec<Span>, bytes: &[u8], start: usize, end: usize) {
    // Highlight `as` as a keyword inside foreach args; everything else as property.
    let mut i = start;
    while i < end {
        if bytes[i].is_ascii_whitespace() {
            let ws = i;
            while i < end && bytes[i].is_ascii_whitespace() {
                i += 1;
            }
            push_span(out, TokenKind::Plain, ws, i);
            continue;
        }
        if i + 2 <= end && &bytes[i..i + 2] == b"as" {
            let boundary_before = i == start || !bytes[i - 1].is_ascii_alphanumeric();
            let boundary_after = i + 2 == end || !bytes[i + 2].is_ascii_alphanumeric();
            if boundary_before && boundary_after {
                push_span(out, TokenKind::Keyword, i, i + 2);
                i += 2;
                continue;
            }
        }
        let s = i;
        while i < end && !bytes[i].is_ascii_whitespace() {
            // stop before standalone `as` handled above on next loop
            if i + 2 <= end
                && &bytes[i..i + 2] == b"as"
                && (i == start || !bytes[i - 1].is_ascii_alphanumeric())
                && (i + 2 == end || !bytes[i + 2].is_ascii_alphanumeric())
            {
                break;
            }
            i += 1;
        }
        if s < i {
            push_span(out, TokenKind::Property, s, i);
        } else {
            i += 1;
        }
    }
}

fn highlight_html_tag(bytes: &[u8], out: &mut Vec<Span>, i: &mut usize) {
    let punct_end = if bytes.get(*i + 1) == Some(&b'/') {
        *i + 2
    } else {
        *i + 1
    };
    push_span(out, TokenKind::Punctuation, *i, punct_end);
    *i = punct_end;
    let tag_start = *i;
    while *i < bytes.len()
        && (bytes[*i].is_ascii_alphanumeric() || bytes[*i] == b'-' || bytes[*i] == b':')
    {
        *i += 1;
    }
    push_span(out, TokenKind::Tag, tag_start, *i);

    while *i < bytes.len() && bytes[*i] != b'>' {
        if bytes[*i].is_ascii_whitespace() {
            let ws = *i;
            while *i < bytes.len() && bytes[*i].is_ascii_whitespace() {
                *i += 1;
            }
            push_span(out, TokenKind::Plain, ws, *i);
            continue;
        }

        if bytes[*i] == b'"' || bytes[*i] == b'\'' {
            let quote = bytes[*i];
            let s = *i;
            *i += 1;
            while *i < bytes.len() && bytes[*i] != quote {
                *i += 1;
            }
            if *i < bytes.len() {
                *i += 1;
            }
            highlight_attr_value(out, s, *i, bytes);
            continue;
        }

        if bytes[*i] == b'=' || bytes[*i] == b'/' {
            push_span(out, TokenKind::Punctuation, *i, *i + 1);
            *i += 1;
            continue;
        }

        let attr_start = *i;
        while *i < bytes.len()
            && !bytes[*i].is_ascii_whitespace()
            && bytes[*i] != b'='
            && bytes[*i] != b'>'
            && bytes[*i] != b'/'
        {
            *i += 1;
        }
        push_span(out, TokenKind::Attribute, attr_start, *i);
    }

    if *i < bytes.len() && bytes[*i] == b'>' {
        push_span(out, TokenKind::Punctuation, *i, *i + 1);
        *i += 1;
    }
}

fn highlight_attr_value(out: &mut Vec<Span>, start: usize, end: usize, bytes: &[u8]) {
    let mut i = start;
    let mut plain = start;
    while i < end {
        if bytes[i..end].starts_with(b"{!!") {
            if plain < i {
                push_span(out, TokenKind::String, plain, i);
            }
            push_span(out, TokenKind::Punctuation, i, i + 3);
            i += 3;
            let expr = i;
            while i + 2 < end && &bytes[i..i + 3] != b"!!}" {
                i += 1;
            }
            push_span(out, TokenKind::Property, expr, i.min(end));
            if i + 2 < end {
                push_span(out, TokenKind::Punctuation, i, i + 3);
                i += 3;
            }
            plain = i;
            continue;
        }
        if bytes[i..end].starts_with(b"{{") {
            if plain < i {
                push_span(out, TokenKind::String, plain, i);
            }
            push_span(out, TokenKind::Punctuation, i, i + 2);
            i += 2;
            let expr = i;
            while i + 1 < end && !(bytes[i] == b'}' && bytes[i + 1] == b'}') {
                i += 1;
            }
            push_span(out, TokenKind::Property, expr, i.min(end));
            if i + 1 < end {
                push_span(out, TokenKind::Punctuation, i, i + 2);
                i += 2;
            }
            plain = i;
            continue;
        }
        i += 1;
    }
    if plain < end {
        push_span(out, TokenKind::String, plain, end);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn highlights_spot_echo_and_directives() {
        let src = r#"@foreach (boards as board)
<a href="{{ board.href }}">{{ board.name }}</a>
@endforeach
@include('assets/header')
{!! raw !!}
"#;
        let spans = highlight_spot_html(src);
        assert!(spans.iter().any(|s| s.kind == TokenKind::Keyword));
        assert!(spans.iter().any(|s| s.kind == TokenKind::Property));
        assert!(spans.iter().any(|s| s.kind == TokenKind::String));
        assert!(spans.iter().any(|s| s.kind == TokenKind::Tag));
    }
}

fn highlight_js_like(source: &str) -> Vec<Span> {
    const KEYWORDS: &[&str] = &[
        "const", "let", "var", "function", "return", "if", "else", "for", "while", "class",
        "import", "export", "from", "async", "await", "new", "this", "true", "false", "null",
        "undefined", "typeof", "instanceof",
    ];

    let bytes = source.as_bytes();
    let mut out = Vec::with_capacity(bytes.len() / 8 + 8);
    let mut i = 0;
    let mut plain = 0usize;

    let flush_plain = |out: &mut Vec<Span>, plain: &mut usize, i: usize| {
        if *plain < i {
            push_span(out, TokenKind::Plain, *plain, i);
            *plain = i;
        }
    };

    while i < bytes.len() {
        if bytes[i] == b'/' && bytes.get(i + 1) == Some(&b'/') {
            flush_plain(&mut out, &mut plain, i);
            let start = i;
            while i < bytes.len() && bytes[i] != b'\n' {
                i += 1;
            }
            push_span(&mut out, TokenKind::Comment, start, i);
            plain = i;
            continue;
        }
        if bytes[i] == b'/' && bytes.get(i + 1) == Some(&b'*') {
            flush_plain(&mut out, &mut plain, i);
            let start = i;
            i += 2;
            while i + 1 < bytes.len() && !(bytes[i] == b'*' && bytes[i + 1] == b'/') {
                i += 1;
            }
            i = (i + 2).min(bytes.len());
            push_span(&mut out, TokenKind::Comment, start, i);
            plain = i;
            continue;
        }
        if bytes[i] == b'"' || bytes[i] == b'\'' || bytes[i] == b'`' {
            flush_plain(&mut out, &mut plain, i);
            let quote = bytes[i];
            let start = i;
            i += 1;
            while i < bytes.len() {
                if bytes[i] == b'\\' {
                    i = (i + 2).min(bytes.len());
                    continue;
                }
                if bytes[i] == quote {
                    i += 1;
                    break;
                }
                i += 1;
            }
            push_span(&mut out, TokenKind::String, start, i);
            plain = i;
            continue;
        }
        if bytes[i].is_ascii_digit() {
            flush_plain(&mut out, &mut plain, i);
            let start = i;
            while i < bytes.len() && (bytes[i].is_ascii_digit() || bytes[i] == b'.') {
                i += 1;
            }
            push_span(&mut out, TokenKind::Number, start, i);
            plain = i;
            continue;
        }
        if bytes[i].is_ascii_alphabetic() || bytes[i] == b'_' || bytes[i] == b'$' {
            let start = i;
            i += 1;
            while i < bytes.len()
                && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_' || bytes[i] == b'$')
            {
                i += 1;
            }
            let word = &source[start..i];
            if KEYWORDS.contains(&word) {
                flush_plain(&mut out, &mut plain, start);
                push_span(&mut out, TokenKind::Keyword, start, i);
                plain = i;
            }
            continue;
        }
        if matches!(
            bytes[i],
            b'{' | b'}' | b'(' | b')' | b'[' | b']' | b';' | b',' | b'.' | b':' | b'='
        ) {
            flush_plain(&mut out, &mut plain, i);
            push_span(&mut out, TokenKind::Punctuation, i, i + 1);
            i += 1;
            plain = i;
            continue;
        }

        i += source[i..].chars().next().map(|c| c.len_utf8()).unwrap_or(1);
    }

    flush_plain(&mut out, &mut plain, i);
    out
}
