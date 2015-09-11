#![deny(warnings)]

mod test;

fn main() {
    test::make_test_source(
        &std::env::current_dir().unwrap()
        );
}
