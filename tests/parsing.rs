#![allow(unstable)]

#[macro_use]
extern crate emojicons;

use emojicons::parse;

#[test]
fn parse_existing() {
    assert_eq!(parse(":smile:"), "\u{01F604}".to_string());
    assert_eq!(parse(":poop:"),  "\u{01F4A9}".to_string());
    assert_eq!(parse(":cat:"),   "\u{01F431}".to_string());
    assert_eq!(parse(":+1:"),    "\u{01F441}".to_string());
    assert_eq!(parse(":-1:"),    "\u{01F44E}".to_string());
    assert_eq!(parse(":8ball:"), "\u{01F3B1}".to_string());
}

#[test]
fn parse_nonexistent() {
    assert_eq!(parse(":stuff:"), ":stuff:".to_string());
    assert_eq!(parse(":++:"),    ":++:".to_string());
    assert_eq!(parse(":--:"),    ":--:".to_string());
    assert_eq!(parse(":666:"),   ":666:".to_string());
}

#[test]
fn parse_in_string() {
    assert_eq!(parse(":cat: make me :smile:"), "\u{01F431} make me \u{01F604}".to_string());
}

#[test]
fn macros() {
    assert_eq!(emoji!(":smile:"), "\u{01F604}".to_string());
    assert_eq!(emoji!(":stuff:"), "".to_string());
}
