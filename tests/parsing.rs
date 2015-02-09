#[macro_use]
extern crate emojicons;

use emojicons::Emojify;

#[test]
fn parse_existing() {
    assert_eq!(":smile:".emojify(), "\u{01F604}");
    assert_eq!(":poop:".emojify(),  "\u{01F4A9}");
    assert_eq!(":cat:".emojify(),   "\u{01F431}");
    assert_eq!(":+1:".emojify(),    "\u{01F441}");
    assert_eq!(":-1:".emojify(),    "\u{01F44E}");
    assert_eq!(":8ball:".emojify(), "\u{01F3B1}");
}

#[test]
fn parse_nonexistent() {
    assert_eq!(":stuff:".emojify(), ":stuff:");
    assert_eq!(":++:".emojify(),    ":++:");
    assert_eq!(":--:".emojify(),    ":--:");
    assert_eq!(":666:".emojify(),   ":666:");
}

#[test]
fn parse_in_string() {
    assert_eq!(":cat: make me :smile:".emojify(), "\u{01F431} make me \u{01F604}");
}

#[test]
fn macros() {
    assert_eq!(emoji!("smile").to_string(), "\u{01F604}".to_string());
}
