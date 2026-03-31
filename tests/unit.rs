use json5_to_json::{convert, detect_indent, IndentOption};
use serde_json::Value;

#[test]
fn basic_conversion() {
    let result = convert("{ hello: 'world' }", false, &IndentOption::Fixed(0)).unwrap();
    assert_eq!(result, r#"{"hello":"world"}"#);
}

#[test]
fn preserves_values() {
    let input = r#"{ a: 1, b: true, c: null, d: "str", e: [1, 2] }"#;
    let result = convert(input, false, &IndentOption::Fixed(0)).unwrap();
    assert_eq!(result, r#"{"a":1,"b":true,"c":null,"d":"str","e":[1,2]}"#);
}

#[test]
fn sort_keys() {
    let input = "{ z: 1, a: 2, m: 3 }";
    let result = convert(input, true, &IndentOption::Fixed(0)).unwrap();
    assert_eq!(result, r#"{"a":2,"m":3,"z":1}"#);
}

#[test]
fn sort_keys_nested() {
    let input = "{ z: { b: 1, a: 2 }, a: 3 }";
    let result = convert(input, true, &IndentOption::Fixed(0)).unwrap();
    assert_eq!(result, r#"{"a":3,"z":{"a":2,"b":1}}"#);
}

#[test]
fn sort_keys_in_arrays() {
    let input = "[{ z: 1, a: 2 }, { y: 3, b: 4 }]";
    let result = convert(input, true, &IndentOption::Fixed(0)).unwrap();
    assert_eq!(result, r#"[{"a":2,"z":1},{"b":4,"y":3}]"#);
}

#[test]
fn no_sort_does_not_error() {
    let input = "{ z: 1, a: 2, m: 3 }";
    let result = convert(input, false, &IndentOption::Fixed(0)).unwrap();
    let v: Value = serde_json::from_str(&result).unwrap();
    assert_eq!(v["z"], 1);
    assert_eq!(v["a"], 2);
    assert_eq!(v["m"], 3);
}

#[test]
fn indent_fixed_2() {
    let result = convert("{ a: 1 }", false, &IndentOption::Fixed(2)).unwrap();
    assert_eq!(result, "{\n  \"a\": 1\n}");
}

#[test]
fn indent_fixed_4() {
    let result = convert("{ a: 1 }", false, &IndentOption::Fixed(4)).unwrap();
    assert_eq!(result, "{\n    \"a\": 1\n}");
}

#[test]
fn indent_zero_compact() {
    let result = convert("{ a: 1, b: 2 }", false, &IndentOption::Fixed(0)).unwrap();
    assert_eq!(result, r#"{"a":1,"b":2}"#);
}

#[test]
fn indent_auto_detects_2_spaces() {
    let input = "{\n  a: 1\n}";
    let result = convert(input, false, &IndentOption::Auto).unwrap();
    assert_eq!(result, "{\n  \"a\": 1\n}");
}

#[test]
fn indent_auto_detects_4_spaces() {
    let input = "{\n    a: 1\n}";
    let result = convert(input, false, &IndentOption::Auto).unwrap();
    assert_eq!(result, "{\n    \"a\": 1\n}");
}

#[test]
fn indent_auto_defaults_to_2() {
    let input = "{a: 1}";
    let result = convert(input, false, &IndentOption::Auto).unwrap();
    assert_eq!(result, "{\n  \"a\": 1\n}");
}

#[test]
fn detect_indent_2_spaces() {
    assert_eq!(detect_indent("{\n  \"a\": 1\n}"), 2);
}

#[test]
fn detect_indent_4_spaces() {
    assert_eq!(detect_indent("{\n    \"a\": 1\n}"), 4);
}

#[test]
fn detect_indent_tab() {
    assert_eq!(detect_indent("{\n\t\"a\": 1\n}"), 1);
}

#[test]
fn detect_indent_no_indentation() {
    assert_eq!(detect_indent("{\"a\": 1}"), 2);
}

#[test]
fn json5_comments_stripped() {
    let input = "{ // comment\n  a: 1 /* block */ }";
    let result = convert(input, false, &IndentOption::Fixed(0)).unwrap();
    assert_eq!(result, r#"{"a":1}"#);
}

#[test]
fn json5_trailing_commas() {
    let input = "{ a: 1, b: 2, }";
    let result = convert(input, false, &IndentOption::Fixed(0)).unwrap();
    assert_eq!(result, r#"{"a":1,"b":2}"#);
}

#[test]
fn json5_unquoted_keys() {
    let input = "{ unquoted: 'value' }";
    let result = convert(input, false, &IndentOption::Fixed(0)).unwrap();
    assert_eq!(result, r#"{"unquoted":"value"}"#);
}

#[test]
fn json5_single_quotes() {
    let input = "{ a: 'single quotes' }";
    let result = convert(input, false, &IndentOption::Fixed(0)).unwrap();
    assert_eq!(result, r#"{"a":"single quotes"}"#);
}

#[test]
fn invalid_input_returns_error() {
    assert!(convert("{ invalid", false, &IndentOption::Fixed(0)).is_err());
}

#[test]
fn indent_option_parse_auto() {
    assert_eq!("auto".parse::<IndentOption>().unwrap(), IndentOption::Auto);
    assert_eq!("AUTO".parse::<IndentOption>().unwrap(), IndentOption::Auto);
}

#[test]
fn indent_option_parse_number() {
    assert_eq!("4".parse::<IndentOption>().unwrap(), IndentOption::Fixed(4));
    assert_eq!("0".parse::<IndentOption>().unwrap(), IndentOption::Fixed(0));
}

#[test]
fn indent_option_parse_invalid() {
    assert!("foo".parse::<IndentOption>().is_err());
}

#[test]
fn indent_option_display() {
    assert_eq!(IndentOption::Auto.to_string(), "auto");
    assert_eq!(IndentOption::Fixed(4).to_string(), "4");
}
