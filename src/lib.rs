#![allow(unstable)]
#![feature(plugin)]

#[plugin] #[no_link]
extern crate phf_mac;
extern crate phf;

#[plugin] #[no_link]
extern crate regex_macros;
extern crate regex;

static REGEX: regex::Regex = regex!(r":([a-zA-Z0-9_\\+\\-]+):");

include!("emojis.rs");

pub fn parse(string: &str) -> String {
    REGEX.replace_all(string, |&: capts: &regex::Captures| {
        /* NOTE: Is this safe to assume? */
        let sym = capts.at(0).unwrap();

        match EMOJIS.get(sym) {
            Some(emoji) => format!("{}", emoji),
            None        => sym.to_string()
        }
    })
}
