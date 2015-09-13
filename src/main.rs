#![deny(warnings)]

#![feature(plugin)]
#![feature(fs_walk)]
#![feature(path_relative_from)]

#![plugin(regex_macros)]

extern crate regex;

mod binunit;

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

    let binunit = binunit::BinUnit::new(&std::env::current_dir().unwrap());
    binunit.run();
}
