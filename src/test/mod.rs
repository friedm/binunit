mod find;
mod parse;
mod gen;


use std::path::PathBuf;

pub fn make_test_source(directory: &PathBuf) -> String {
    gen::generate_test(
        &parse::parse_testfn_list(
            &find::recursive_read(directory)
            )
        )
}
