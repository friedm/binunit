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
use std::str;

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

    pub fn run(&self) -> Result<String, String> {

        let generated_src = self.make_test_source();

        let test_targets = find::DirWalker::new(&self.exec_dir, 
            regex!(r"^(o|a|so)$"));
        let test_targets = test_targets.walk_map(|path| {
            path.relative_from(&self.exec_dir).unwrap().to_str().unwrap().to_owned()
            });

        let work_dir = build::WorkingDir::new(".binunit_tmp", &self.exec_dir);

        work_dir.write_to_tmp(&generated_src);
        match work_dir.build(&test_targets) {

            Ok(status) => match status.code() {
                Some(0) => {
                    match work_dir.run() {
                        Ok(output) => match output.status.code() {
                            Some(0) => match str::from_utf8(&output.stdout[..]) {
                                Ok(val) => Ok(val.to_owned()),
                                Err(err) => Err(format!("unable to interpret test output as utf8: {}", err))
                            },
                            Some(code) => Err(format!("test executable returned nonzero exit status: {}", code)),
                            None => Err("test executable command failed".to_owned())
                        },
                        Err(e) => Err(format!("test executable failed: {}", e))
                    }
                },
                Some(code) => Err(format!("gcc returned nonzero exit status: {}", code)),
                None => Err("gcc command failed".to_owned())
            },
            Err(e) => Err(format!("gcc command failed: {}\n\tgcc may be missing", e))
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
