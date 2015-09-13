#![deny(warnings)]

#![feature(plugin)]
#![feature(fs_walk)]
#![feature(path_relative_from)]

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

    let test_targets = test::find::DirWalker::new(&std::env::current_dir().unwrap(), 
        regex!(r"^(o|a|so)&"));

    let work_dir = build::WorkingDir::new(".punit_tmp");

    work_dir.write_to_tmp(&generated_src);
    match work_dir.build(&test_targets.walk_map(|path| path.relative_from(&std::env::current_dir().unwrap()).unwrap().to_str().unwrap().to_owned())) {
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
