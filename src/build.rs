use std::env;
use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::io;
use std::path::PathBuf;
use std::process::Command;
use std::process::ExitStatus;


pub fn write_to_tmp(generated_src: &String) {

    let tmp_dir = setup_tmp_dir(".punit_tmp");

    write_to(&tmp_dir.join("punit.c"), generated_src);
    write_to(&tmp_dir.join("punit.h"), &include_str!("../csrc/punit.h").to_owned());
    write_to(&tmp_dir.join("punit_main.c"), &include_str!("../csrc/punit_main.c").to_owned());
}

fn setup_tmp_dir(dir_name: &'static str) -> PathBuf {

    let mut dir = env::current_dir().unwrap();
    dir.push(dir_name);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn write_to(file_path: &PathBuf, to_write: &String) {

    let mut f = File::create(file_path).unwrap();
    f.write_all(&to_write.clone().into_bytes()[..]).unwrap();
    f.sync_data().unwrap();
}

pub fn build() -> Result<ExitStatus, io::Error> {

    Command::new("gcc")
        .current_dir(&setup_tmp_dir(".punit_tmp"))
        .arg("-o")
        .arg("punit")
        .arg("punit_main.c")
        .arg("punit.c")
        .arg("--entry=punit_main")
        .arg("-nostartfiles")
        .status()
}

#[cfg(test)]
mod test {
    use std::fs;
    use std::fs::File;
    use std::env;
    use std::path::PathBuf;

    #[test]
    fn setup_tmp() {

        let dir_name = ".test_setup_tmp";
        let dir = dir(dir_name);

        fs::remove_dir_all(dir.clone()).unwrap_or(());
        assert!(!dir_exists(&dir));

        super::setup_tmp_dir(dir_name);
        assert!(dir_exists(&dir));
    }

    fn dir(name: &'static str) -> PathBuf {

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

        let test_file = dir(".test_write_to").join("write_to.test");
        fs::remove_file(test_file.clone()).unwrap_or(());
        assert!(!file_exists(&test_file));

        fs::create_dir_all(".test_write_to").unwrap();

        super::write_to(&test_file, &"this string is written to file".to_owned());

        assert!(file_exists(&test_file));
    }

    #[test]
    fn build() {

        super::write_to_tmp(&"void punit_run_tests(void){}\n".to_owned());
        super::build().unwrap();
        assert!(file_exists(&dir(".punit_tmp").join("punit")));
    }
}
