#![deny(warnings)]

extern crate binunit;

fn main() {

    let binunit = binunit::BinUnit::new(&std::env::current_dir().unwrap());
    let output = binunit.run().unwrap_or_else(|err| {println!("{}", err); vec!["\nbinunit failed".to_owned()]});

    output.iter()
        .inspect(|elem| println!("{}", elem))
        .collect::<Vec<_>>();
}
