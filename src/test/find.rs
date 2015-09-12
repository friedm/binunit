use std::io::prelude::*;
use std::fs;
use std::path::PathBuf;

extern crate regex;

pub fn recursive_read(dir_path: &PathBuf) -> Vec<String > {
    walk_find_source_files(dir_path)
        .iter()
        .map(|name| load(&name))
        .collect()
}

fn walk_find_source_files(dir_path: &PathBuf) -> Vec<PathBuf> {
    fs::walk_dir(dir_path).unwrap()
        .map(|path| path.unwrap().path())
        .filter(|path| !is_hidden_path(&path))
        .filter(|path| has_src_extension(&path))
        .collect()
}

fn is_hidden_path(path: &PathBuf) -> bool {
    let hidden_path_regex = regex!(r"/\.");

    let path = path.clone().into_os_string();
    let path = path.to_str().unwrap();

    hidden_path_regex.is_match(path)
}

fn has_src_extension(path: &PathBuf) -> bool {
    let supported_src_extensions = regex!(r"c");

    match path.extension() {
        Some(ext) =>
            supported_src_extensions.is_match(ext.to_str().unwrap()),
        None => false
    }
}

fn load(path: &PathBuf) -> String {
    let mut f = fs::File::open(path).unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    contents
}

#[cfg(test)]
mod test {
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
