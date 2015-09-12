#![feature(plugin)]
#![plugin(regex_macros)]
#![feature(fs_walk)]
//#![deny(warnings)]
//

extern crate regex;

mod test;

fn main() {
    test::make_test_source(
        &std::env::current_dir().unwrap()
        );
}
