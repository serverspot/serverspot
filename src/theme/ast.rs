use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Text(String),
    /// Escaped interpolation `{{ expr }}`
    Interp {
        expr: String,
        raw: bool,
    },
    Element {
        tag: String,
        attrs: Vec<(String, AttrValue)>,
        children: Vec<Node>,
        self_closing: bool,
    },
    If {
        arms: Vec<(String, Vec<Node>)>,
        else_body: Vec<Node>,
    },
    ForEach {
        collection: String,
        item: String,
        body: Vec<Node>,
    },
    Include {
        path: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum AttrValue {
    Literal(String),
    Interp { expr: String, raw: bool },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Template {
    pub nodes: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TemplateValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<TemplateValue>),
    Object(BTreeMap<String, TemplateValue>),
}

impl TemplateValue {
    pub fn as_bool(&self) -> bool {
        match self {
            Self::Null => false,
            Self::Bool(b) => *b,
            Self::Number(n) => *n != 0.0,
            Self::String(s) => !s.is_empty(),
            Self::Array(a) => !a.is_empty(),
            Self::Object(o) => !o.is_empty(),
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            Self::Null => String::new(),
            Self::Bool(b) => b.to_string(),
            Self::Number(n) => {
                if n.fract() == 0.0 {
                    format!("{}", *n as i64)
                } else {
                    n.to_string()
                }
            }
            Self::String(s) => s.clone(),
            Self::Array(a) => format!("{} items", a.len()),
            Self::Object(_) => String::from("[object]"),
        }
    }

    pub fn get_path_ref(&self, path: &str) -> Option<&TemplateValue> {
        let mut cur = self;
        for part in path.split('.').filter(|p| !p.is_empty()) {
            cur = match cur {
                Self::Object(map) => map.get(part)?,
                Self::Array(items) => {
                    let idx = part.parse::<usize>().ok()?;
                    items.get(idx)?
                }
                _ => return None,
            };
        }
        Some(cur)
    }
}

impl From<&str> for TemplateValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for TemplateValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<bool> for TemplateValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<i32> for TemplateValue {
    fn from(value: i32) -> Self {
        Self::Number(value as f64)
    }
}

impl From<u32> for TemplateValue {
    fn from(value: u32) -> Self {
        Self::Number(value as f64)
    }
}

impl From<usize> for TemplateValue {
    fn from(value: usize) -> Self {
        Self::Number(value as f64)
    }
}
