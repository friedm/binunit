#![deny(warnings)]

#![feature(plugin)]
#![feature(fs_walk)]
#![feature(path_relative_from)]

#![plugin(regex_macros)]

pub mod find;
mod parse;
mod gen;
mod build;

extern crate regex;

use std::path::PathBuf;

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

pub struct BinUnit {
    exec_dir: PathBuf
}

impl BinUnit {
    pub fn new(exec_dir: &PathBuf) -> BinUnit {

        BinUnit {
            exec_dir: exec_dir.clone()
        }
    }

    pub fn run(&self) {

        let generated_src = self.make_test_source();

        let test_targets = find::DirWalker::new(&self.exec_dir, 
            regex!(r"^(o|a|so)&"));

        let work_dir = build::WorkingDir::new(".binunit_tmp");

        work_dir.write_to_tmp(&generated_src);
        match work_dir.build(&test_targets.walk_map(|path| {
            path.relative_from(&self.exec_dir).unwrap().to_str().unwrap().to_owned()
            })) {

            Ok(status) => match status.code() {
                Some(0) => {
                    match work_dir.run() {
                        Ok(status) => match status.code() {
                            Some(0) => (),
                            Some(code) => println!("test executable returned nonzero exit status: {}", code),
                            None => println!("test executable command failed")
                        },
                        Err(e) => println!("test executable failed: {}", e)
                    }
                },
                Some(code) => println!("gcc returned nonzero exit status: {}", code),
                None => println!("gcc command failed")
            },
            Err(e) => println!("gcc command failed: {}\n\tgcc may be missing", e)
        }
    }

    fn make_test_source(&self) -> String {

        gen::generate_test(
            &parse::parse_testfn_list(
                &find::DirWalker::new(&self.exec_dir, regex!(r"c"))
                .walk_map(|path| find::load(&path))
                )
            )
    }

}
