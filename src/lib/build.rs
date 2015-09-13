use std::env;
use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::io;
use std::fmt;
use std::path::PathBuf;
use std::process::{Command, ExitStatus, Output};


pub struct WorkingDir {
    work_dir: PathBuf,
    exec_dir: PathBuf 
}

impl WorkingDir {
    pub fn new(work_dir: &str, exec_dir: &PathBuf) -> WorkingDir {

        WorkingDir {
            work_dir: Self::setup_tmp_dir(&work_dir.to_owned()[..]),
            exec_dir: exec_dir.clone()
        }
    }

    fn setup_tmp_dir(dir_name: &str) -> PathBuf {

        let mut dir = env::current_dir().unwrap();
        dir.push(dir_name);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    pub fn write_to_tmp(&self, generated_src: &String) {

        Self::write_to(&self.work_dir.join("binunit_gen.c"), generated_src);
        Self::write_to(&self.work_dir.join("binunit.h"), &include_str!("../../csrc/binunit.h").to_owned());
        Self::write_to(&self.work_dir.join("binunit_main.c"), &include_str!("../../csrc/binunit_main.c").to_owned());
        Self::write_to(&self.work_dir.join("binunit_runtime.h"), &include_str!("../../csrc/binunit_runtime.h").to_owned());
        Self::write_to(&self.work_dir.join("binunit_runtime.c"), &include_str!("../../csrc/binunit_runtime.c").to_owned());
    }

    fn write_to(file_path: &PathBuf, to_write: &String) {

        let mut f = File::create(file_path).unwrap();
        f.write_all(&to_write.clone().into_bytes()[..]).unwrap();
        f.sync_data().unwrap();
    }

    pub fn build(&self, test_targets: &Vec<String>) -> Result<ExitStatus, io::Error> {

        Self::debug("targets", test_targets);
        Command::new("gcc")
            .current_dir(&self.exec_dir)
            .arg("-o")
            .arg(&self.work_dir.join("binunit").to_str().unwrap())
            .arg(&self.work_dir.join("binunit_main.c").to_str().unwrap())
            .arg(&self.work_dir.join("binunit_runtime.c").to_str().unwrap())
            .arg(&self.work_dir.join("binunit_gen.c").to_str().unwrap())
            .args(&test_targets[..])
            .arg("--entry=binunit_main")
            .arg("-nostartfiles")
            .status()
    }

    fn debug<T>(label: &'static str, debug: T) where T : fmt::Debug {
        println!("{}: {:?}", label, debug);
    }

    pub fn run(&self) -> Result<Output, io::Error> {
        
        Command::new(&self.work_dir.join("binunit").to_str().unwrap())
            .current_dir(&self.exec_dir)
            .output()
    }
}

#[cfg(test)]
mod test {
    use std::fs;
    use std::fs::File;
    use std::env;
    use std::path::PathBuf;
    use ToOwnedStringVec;

    #[test]
    fn setup_tmp() {

        let dir_name = ".test_setup_tmp";
        let dir = make_dir(dir_name);

        fs::remove_dir_all(dir.clone()).unwrap_or(());
        assert!(!dir_exists(&dir));

        super::WorkingDir::setup_tmp_dir(dir_name);
        assert!(dir_exists(&dir));
    }

    fn make_dir(name: &'static str) -> PathBuf {

        env::current_dir().unwrap().join(name)
    }

    fn dir_exists(path: &PathBuf) -> bool {

        match File::create(path.join("file.test")) {
            Ok(_) => true,
            Err(_) => false
        }
    }

    fn file_exists(path: &PathBuf) -> bool {

        match File::open(path) {
            Ok(_) => true,
            Err(_) => false
        }
    }

    #[test]
    fn write_to() {

        let test_file = make_dir(".test_write_to").join("write_to.test");
        fs::remove_file(test_file.clone()).unwrap_or(());
        assert!(!file_exists(&test_file));

        fs::create_dir_all(".test_write_to").unwrap();

        super::WorkingDir::write_to(&test_file, &"this string is written to file".to_owned());

        assert!(file_exists(&test_file));
    }

    #[test]
    fn build() {

        let dir = super::WorkingDir::new(".binunit_tmp", &make_dir("."));
        dir.write_to_tmp(&"void binunit_run_tests(void){}\n".to_owned());
        dir.build(&Vec::new()).unwrap();
        assert!(file_exists(&make_dir(".binunit_tmp").join("binunit")));
    }

    #[test]
    fn run() {

        let dir = super::WorkingDir::new(".test_run", &make_dir("."));
        dir.write_to_tmp(&"void binunit_run_tests(void){}\n".to_owned());
        dir.build(&vec!["tests/testc/passfail.c"].to_owned_vec()).unwrap();
        dir.run().unwrap();
    }
}

