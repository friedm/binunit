#![deny(warnings)]

extern crate binunit;

fn main() {

    let binunit = binunit::BinUnit::new(&std::env::current_dir().unwrap());
    binunit.run();
}
