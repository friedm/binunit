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

    pub fn run(&self) -> Result<Vec<String>, String> {

        let label_list = Self::make_label_list(&self.exec_dir);
        let test_targets = Self::make_target_list(&self.exec_dir);

        let generated_src = Self::make_test_source(&label_list);

        let work_dir = build::WorkingDir::new(".binunit_tmp", &self.exec_dir);
        work_dir.write_to_tmp(&generated_src);

        Self::build_and_handle(&work_dir, &test_targets)
            .and_then(|_| Self::run_all_tests(&work_dir, &label_list))
    }

    fn make_label_list(exec_dir: &PathBuf) -> Vec<String> {

        parse::parse_testfn_list(
            &find::DirWalker::new(&exec_dir, regex!(r"c"))
            .walk_map(|path| find::load(&path))
            )
    }

    fn make_target_list(exec_dir: &PathBuf) -> Vec<String> {

        find::DirWalker::new(&exec_dir, regex!(r"^(o|a|so)$"))
            .walk_map(|path| {
                path.relative_from(&exec_dir).unwrap().to_str().unwrap().to_owned()
            })
    }

    fn make_test_source(labels: &Vec<String>) -> String {

        gen::generate_test(
            labels
           )
    }

    fn build_and_handle(work_dir: &build::WorkingDir, test_targets: &Vec<String>) 
        -> Result<String, String> {

        match work_dir.build(&test_targets) {
            Ok(status) => match status.code() {
                Some(0) => Ok("build successful".to_owned()),
                Some(code) => Err(format!("gcc returned nonzero exit status: {}", code)),
                None => Err("gcc command failed".to_owned())
            },
            Err(e) => Err(format!("gcc command failed: {}\n\tgcc may be missing", e))
        }
    }

    fn run_all_tests(work_dir: &build::WorkingDir, label_list: &Vec<String>) 
        -> Result<Vec<String>, String> {

        label_list.iter().map(|label| Self::run_one_and_handle(&label, &work_dir))
            .fold(Ok(Vec::new()), |a, b| { 
                match a {
                    Ok(mut a) => match b {
                        Ok(b) => { a.push(b); Ok(a) },
                        Err(e) => Err(e)
                    },
                    Err(e) => Err(e)
                }
            })
    }

    fn run_one_and_handle(label: &String, work_dir: &build::WorkingDir)
        -> Result<String, String> {

        match work_dir.run(&label) {
            Ok(output) => match output.status.code() {
                Some(0) => match str::from_utf8(&output.stdout[..]) {
                    Ok(val) => Ok(val.to_owned()),
                    Err(err) => Err(format!("unable to interpret test output as utf8: {}", err))
                },
                Some(code) => Err(format!("test executable returned nonzero exit status: {}", code)),
                None => Ok(format!("{}: failed (segfault)\n", label))
            },
            Err(e) => Err(format!("test executable failed: {}", e))
        }
    }
}
