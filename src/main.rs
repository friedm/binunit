#![deny(warnings)]

#![feature(plugin)]
#![feature(fs_walk)]

#![plugin(regex_macros)]


extern crate regex;

mod test;

use std::io::prelude::*;
use std::fs::File;

trait ToOwnedStringVec {
    fn to_owned_vec(&self) -> Vec<String>;
}

impl ToOwnedStringVec for Vec<&'static str> {
    fn to_owned_vec(&self) -> Vec<String> {
        self.iter()
            .map(|&item| item.to_owned())
            .collect()
    }
}

fn main() {
    let src = test::make_test_source(
        &std::env::current_dir().unwrap()
        );

    let mut f = File::create("punit.c").unwrap();
    f.write_all(&src.into_bytes()[..]).unwrap();
}
