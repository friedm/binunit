#![deny(warnings)]

#![feature(plugin)]
#![feature(fs_walk)]

#![plugin(regex_macros)]

extern crate regex;

mod test;
mod build;

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

    let generated_src = test::make_test_source(
        &std::env::current_dir().unwrap()
        );

    build::write_to_tmp(&generated_src);
    match build::build() {
        Ok(status) => match status.code() {
            Some(0) => (),
            Some(code) => println!("gcc returned nonzero exit status: {}", code),
            None => println!("gcc command failed")
            },
        Err(e) => println!("gcc command failed: {}", e)
    }
}
