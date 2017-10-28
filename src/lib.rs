extern crate phf;
extern crate regex;

use std::fmt;
use regex::{
    Regex,
    Captures,
};

include!(concat!(env!("OUT_DIR"), "/emojis.rs"));

/// Macro for compile-time emoji lookup
///
/// This macro will expand to the string stored in `EMOJIS` on compile-time.
/// This doesn't introduce any overhead, but is useful to prevent pasting of
/// unicode into the code.
///
/// # Example
///
/// ```rust
/// #[macro_use] extern crate emojicons;
/// 
/// # fn main() {
/// assert_eq!(emoji!("cat").to_string(), "\u{01F431}");
/// # }
/// ```
#[macro_export]
macro_rules! emoji {
    ($e: expr) => (
        $crate::EMOJIS.get(&format!(":{}:", $e)[..]).unwrap_or(&$e);
    )
}

/// Newtype used for substituting emoji codes for emoji
///
/// Leaves the notation intact if a corresponding emoji is not found in the
/// lookup table.
pub struct EmojiFormatter<'a>(pub &'a str);

impl<'a> std::fmt::Display for EmojiFormatter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let re = Regex::new(r":([a-zA-Z0-9_+-]+):").unwrap();
        
        let result = re.replace_all(self.0, |capts: &Captures| {
            let sym = capts.at(0).unwrap();

            match EMOJIS.get(sym) {
                Some(e) => format!("{}", e),
                None    => sym.to_string()
            }
        });

        write!(f, "{}", result)
    }
}

