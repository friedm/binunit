use std::io::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

extern crate regex;

pub fn recursive_read(dir_path: &PathBuf) -> Vec<String > {
    walk_find_source_files(dir_path)
        .iter()
        .map(|name| load(&name))
        .collect()
}

fn walk_find_source_files(dir_path: &PathBuf) -> Vec<PathBuf> {
    let mut files = Vec::new();

    let hidden_path_regex = regex!(r"/\.");
    let supported_src_extensions = regex!(r"c|rs");

    for path in fs::walk_dir(dir_path).unwrap() {
        let path = path.unwrap().path();
        let path_os_string = path.clone().into_os_string();
        let path_string = path_os_string.to_str().unwrap();

        if hidden_path_regex.is_match(path_string) {
            continue;
        }

        match path.extension() {
            Some(ext) => {
                if supported_src_extensions.is_match(ext.to_str().unwrap()) {
                    files.push(path.clone());
                }
            },
            None => ()
        }
    }



    files
}

fn load(path: &PathBuf) -> String {
    let mut f = fs::File::open(path).unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    contents
}

#[cfg(test)]
mod test {
    use std::fs;
    use std::path;

    #[test]
    fn find_source() {
        let files = super::walk_find_source_files(&path::PathBuf::from("./test"));

        assert_eq!(1, files.len());
        assert!(files.contains(&path::PathBuf::from("./test/main.c")));
    }

    #[test]
    fn recursive_read() {
        let files = super::recursive_read(&path::PathBuf::from("./test"));

        assert_eq!(1, files.len());

        assert!(files.contains(&super::load(&path::PathBuf::from("./test/main.c"))));
    }

    #[test]
    fn load() {
        let contents = super::load(&path::PathBuf::from("./test/main.c"));
        assert_eq!(String::from("///[test]\nvoid test_fn(void) {\n}\n"), contents);
    }
}
