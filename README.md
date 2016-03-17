# Emojicons [![Build Status](https://travis-ci.org/jiri/rust-emojicons.svg?branch=master)](https://travis-ci.org/jiri/rust-emojicons)

Emojicons is a simple emoji parser written in Rust focused on ease of use and speed. It uses hashed map for fast lookup and compiled regular expressions for parsing strings.

## Example usage

The library is extremely straightforward to use. For transforming strings, use the formatter:

```rust
format!("{}", EmojiFormatter("Hello, :smile:!"));
```

This will return "Hello! :smile:"

There is also a macro for direct access to emoji:

```rust
emoji!("cat");
```

Will return a string with the glyph for :cat:.
