#![deny(warnings)]

#![feature(plugin)]
#![feature(fs_walk)]

#![plugin(regex_macros)]


extern crate regex;

mod test;

fn main() {
    test::make_test_source(
        &std::env::current_dir().unwrap()
        );
}
