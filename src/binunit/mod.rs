pub mod find;
mod parse;
mod gen;
mod build;

use std::path::PathBuf;

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

        let work_dir = build::WorkingDir::new(".punit_tmp");

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
