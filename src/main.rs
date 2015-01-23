#![allow(unstable)]

extern crate emoji;

fn main() {
    let args = std::os::args();

    if args.len() < 2 {
        return;
    }

    let string = args[1].clone();

    println!("{}", emoji::parse(string));
}
