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

static REGEX: Regex = regex!(r":([a-zA-Z0-9_\\+\\-]+):");

include!("emojis.rs");

#[macro_export]
macro_rules! emoji {
    ($e: expr) => (
        $crate::EMOJIS.get($e.as_slice()).unwrap_or(&"").to_string()
    )
}

pub fn parse(string: String) -> String {
    REGEX.replace_all(string.as_slice(), |&: capts: &Captures| {
        let sym = capts.at(0).unwrap();

        match EMOJIS.get(sym) {
            Some(e) => format!("{}", e),
            None    => sym.to_string()
        }
    })
}
