
/// Generate a source file for the test executable
/// from a list of test function names.
pub fn generate_test(fn_list: &Vec<String>) -> String {

    format!("{}\n\nvoid main(void) {{\n{}\n}}\n", 
            fn_list.map_and_concat(|label| format!("\nvoid {}(void);", label)),
            fn_list.map_and_concat(|label| format!("\n\t{}();", label)))
        .to_owned()
}

trait MapConcat {
    fn map_and_concat<F>(&self, mapper: F) -> String 
        where F : Fn(&String) -> String;
}

impl MapConcat for Vec<String> {
    fn map_and_concat<F>(&self, mapper: F) -> String 
        where F : Fn(&String) -> String {

        self.iter()
            .map(mapper)
            .fold(String::new(), |a,b| format!("{}{}", a, b))
    }
}

#[cfg(test)]
mod test {
    use ToOwnedStringVec;

    #[test]
    fn main_test() {

        assert_generated_contains("void main(void)", test_fn_list());
    }

    fn assert_generated_contains(contents: &str, fn_list: Vec<&'static str>) {

        let actual = super::generate_test(&fn_list.to_owned_vec());
        println!("generated: {}", actual);
        assert!(actual.contains(contents));
    }

    fn test_fn_list() -> Vec<&'static str> {

        vec!["fn1", "fn2"]
    }

    #[test]
    fn call_test() {

        for fn_id in test_fn_list() {
            assert_generated_contains(&format!("{}();", fn_id)[..], test_fn_list());
        }
    }

    #[test]
    fn define_test() {

        for fn_id in test_fn_list() {
            assert_generated_contains(&format!("void {}(void);", fn_id)[..], test_fn_list());
        }
    }
}
