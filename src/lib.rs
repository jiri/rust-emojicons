#![allow(unstable)]
#![feature(plugin)]

#[plugin] #[no_link]
extern crate phf_mac;
extern crate phf;

#[plugin] #[no_link]
extern crate regex_macros;
extern crate regex;

use regex::{
    Regex,
    Captures,
};

static REGEX: Regex = regex!(r":([a-zA-Z0-9_+-]+):");

include!("emojis.rs");

/// Macro for accessing emojis directly
///
/// This macro will expand to the string stored in `EMOJIS`, yielding a `String`.
///
/// If the emoji does not exist, "" will be returned instead.
///
/// # Example
///
/// ```rust
/// #[macro_use] extern crate emojicons;
///
/// # fn main() {
/// emoji!(":smile:");
/// # }
/// ```
#[macro_export]
macro_rules! emoji {
    ($e: expr) => (
        $crate::EMOJIS.get($e.as_slice()).unwrap_or(&"").to_string()
    )
}

/// An extension trait for `str` which provides emojifying methods
pub trait Emojify {
    fn emojify(&self) -> String;
}

impl Emojify for str {
    /// Parse the text, replacing emoji notation with a unicode character
    ///
    /// Leaves the notation intact if a corresponding emoji is not found in the
    /// lookup table.
    ///
    /// # Example
    ///
    /// ```rust
    /// use emojicons::Emojify;
    ///
    /// "Hello, :poop:!".emojify();
    /// ```
    fn emojify(&self) -> String {
        REGEX.replace_all(self, |&: capts: &Captures| {
            let sym = capts.at(0).unwrap();

            match EMOJIS.get(sym) {
                Some(e) => format!("{}", e),
                None    => sym.to_string()
            }
        })
    }
}
