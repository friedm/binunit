#![deny(warnings)]

extern crate regex;

use regex::Regex;

fn main() {
    let re = Regex::new(r"").unwrap();
    println!("stuff!");
}
