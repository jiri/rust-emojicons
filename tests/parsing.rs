#[macro_use] extern crate emojicons;

use emojicons::EmojiFormatter;

#[test]
fn parse_existing() {
    assert_eq!(format!("{}", EmojiFormatter(":smile:")), "\u{01F604}");
    assert_eq!(format!("{}", EmojiFormatter(":poop:")),  "\u{01F4A9}");
    assert_eq!(format!("{}", EmojiFormatter(":cat:")),   "\u{01F431}");
    assert_eq!(format!("{}", EmojiFormatter(":+1:")),    "\u{01F441}");
    assert_eq!(format!("{}", EmojiFormatter(":-1:")),    "\u{01F44E}");
    assert_eq!(format!("{}", EmojiFormatter(":8ball:")), "\u{01F3B1}");
}

#[test]
fn parse_nonexistent() {
    assert_eq!(format!("{}", EmojiFormatter(":stuff:")), ":stuff:");
    assert_eq!(format!("{}", EmojiFormatter(":++:")),    ":++:");
    assert_eq!(format!("{}", EmojiFormatter(":--:")),    ":--:");
    assert_eq!(format!("{}", EmojiFormatter(":666:")),   ":666:");
}

#[test]
fn format_string() {
    assert_eq!(format!("{}", EmojiFormatter(":cat: make me :smile:")), "\u{01F431} make me \u{01F604}");
}

#[test]
fn macros() {
    assert_eq!(emoji!("smile").to_string(), "\u{01F604}");
}

#[test]
fn macros_nonexistent() {
    assert_eq!(emoji!("eat").to_string(), "eat");
}
