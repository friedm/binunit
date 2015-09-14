use std::io::prelude::*;
use std::fs;
use std::path::PathBuf;

extern crate regex;

pub struct DirWalker {
    path: PathBuf,
    extension_regex: regex::Regex
}

impl DirWalker {
    pub fn new(path: &PathBuf, regex: regex::Regex) -> DirWalker {

        DirWalker {
            path: path.to_owned(),
            extension_regex: regex
        }
    }

    pub fn walk_map<F>(&self, mapper: F) -> Vec<String > 
        where F : Fn(&PathBuf) -> String {

        self.walk_and_filter()
            .iter()
            .map(|&ref name| mapper(name))
            .collect()
    }

    fn walk_and_filter(&self) -> Vec<PathBuf> {

        fs::walk_dir(&self.path).unwrap()
            .map(|path| path.unwrap().path())
            .filter(|path| !Self::is_hidden_path(&path))
            .filter(|path| self.matches_extension(&path))
            .collect()
    }

    fn is_hidden_path(path: &PathBuf) -> bool {

        let hidden_path_regex = regex!(r"/\.");

        let path = path.clone().into_os_string();
        let path = path.to_str().unwrap();

        hidden_path_regex.is_match(path)
    }

    fn matches_extension(&self, path: &PathBuf) -> bool {

        match path.extension() {
            Some(ext) =>
                self.extension_regex
                    .is_match(ext.to_str().unwrap()),
                None => false
        }
    }
}

pub fn load(path: &PathBuf) -> String {
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

        let files = test_walker().walk_and_filter();

        assert_eq!(1, files.len());
        assert!(files.contains(&path::PathBuf::from("./Cargo.toml")));
    }

    fn test_walker() -> super::DirWalker {

        super::DirWalker::new(&path::PathBuf::from("."), regex!(r"toml"))
    }

    #[test]
    fn walk_map() {

        let files = test_walker().walk_map(|path| super::load(&path));

        assert_eq!(1, files.len());
        assert!(files.contains(&super::load(&path::PathBuf::from("./Cargo.toml"))));
    }

    #[test]
    fn load() {

        let contents = super::load(&path::PathBuf::from("./tests/testc/passfail.c"));
        assert!(contents.contains("///[test]\nvoid test_pass(void) {\n}\n"));
    }
}
