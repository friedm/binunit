pub mod find;
mod parse;
mod gen;

use std::path::PathBuf;

pub fn make_test_source(directory: &PathBuf) -> String {

    gen::generate_test(
        &parse::parse_testfn_list(
            &find::DirWalker::new(directory, regex!(r"c"))
                .walk_map(|path| find::load(&path))
            )
        )
}
