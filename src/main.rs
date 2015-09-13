#![deny(warnings)]

extern crate binunit;

fn main() {

    let binunit = binunit::BinUnit::new(&std::env::current_dir().unwrap());
    let output = binunit.run().unwrap_or_else(|err| {println!("{}", err); "\nbinunit failed".to_owned()});
    println!("{}", output);
}
