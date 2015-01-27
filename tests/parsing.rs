#![allow(unstable)]

#[macro_use]
extern crate emojicons;

use emojicons::parse;
use emojicons::Emojify;

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

#[test]
fn new_syntax() {
    assert_eq!(":smile:".emojify(), "\u{01F604}");
    assert_eq!(":poop:".emojify(),  "\u{01F4A9}");
    assert_eq!(":cat:".emojify(),   "\u{01F431}");
    assert_eq!(":+1:".emojify(),    "\u{01F441}");
    assert_eq!(":-1:".emojify(),    "\u{01F44E}");
    assert_eq!(":8ball:".emojify(), "\u{01F3B1}");
}
