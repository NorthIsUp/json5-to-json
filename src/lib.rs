use anyhow::Result;
use serde::Serialize;
use serde_json::{Map, Value};
use std::{fmt, str::FromStr};

#[derive(Clone, Debug, PartialEq)]
pub enum IndentOption {
    Auto,
    Fixed(usize),
}

impl FromStr for IndentOption {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case("auto") {
            Ok(IndentOption::Auto)
        } else {
            s.parse::<usize>()
                .map(IndentOption::Fixed)
                .map_err(|_| format!("expected a number or \"auto\", got \"{s}\""))
        }
    }
}

impl fmt::Display for IndentOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IndentOption::Auto => write!(f, "auto"),
            IndentOption::Fixed(n) => write!(f, "{n}"),
        }
    }
}

pub fn detect_indent(input: &str) -> usize {
    for line in input.lines().skip(1) {
        let stripped = line.trim_start_matches(' ');
        let spaces = line.len() - stripped.len();
        if spaces > 0 {
            return spaces;
        }
        let stripped = line.trim_start_matches('\t');
        let tabs = line.len() - stripped.len();
        if tabs > 0 {
            return tabs;
        }
    }
    2
}

pub fn sort_value(value: &mut Value) {
    match value {
        Value::Object(map) => {
            let mut sorted = Map::new();
            let mut keys: Vec<String> = map.keys().cloned().collect();
            keys.sort();
            for key in keys {
                let mut v = map.remove(&key).unwrap();
                sort_value(&mut v);
                sorted.insert(key, v);
            }
            *map = sorted;
        }
        Value::Array(arr) => {
            for v in arr {
                sort_value(v);
            }
        }
        _ => {}
    }
}

pub fn convert(input: &str, sort: bool, indent: &IndentOption) -> Result<String> {
    let mut value: Value = json5::from_str(input)?;

    if sort {
        sort_value(&mut value);
    }

    let indent = match indent {
        IndentOption::Auto => detect_indent(input),
        IndentOption::Fixed(n) => *n,
    };

    let output = if indent == 0 {
        serde_json::to_string(&value)?
    } else {
        let buf = Vec::new();
        let indent_bytes = " ".repeat(indent).into_bytes();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(&indent_bytes);
        let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
        value.serialize(&mut ser)?;
        String::from_utf8(ser.into_inner())?
    };

    Ok(output)
}
