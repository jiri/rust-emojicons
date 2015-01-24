#![allow(unstable)]

extern crate emoji;

use emoji::parse;

#[test]
fn parse_existing() {
    assert_eq!(parse(":smile:".to_string()), "\u{01F604}".to_string());
    assert_eq!(parse(":poop:".to_string()),  "\u{01F4A9}".to_string());
    assert_eq!(parse(":cat:".to_string()),   "\u{01F431}".to_string());
    assert_eq!(parse(":+1:".to_string()),    "\u{01F441}".to_string());
    assert_eq!(parse(":-1:".to_string()),    "\u{01F44E}".to_string());
    assert_eq!(parse(":8ball:".to_string()), "\u{01F3B1}".to_string());
}

#[test]
fn parse_nonexistent() {
    assert_eq!(parse(":stuff:".to_string()), ":stuff:".to_string());
    assert_eq!(parse(":++:".to_string()),    ":++:".to_string());
    assert_eq!(parse(":--:".to_string()),    ":--:".to_string());
    assert_eq!(parse(":666:".to_string()),   ":666:".to_string());
}
