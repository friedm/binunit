
/// Generate a source file for the test executable
/// from a list of test function names.
pub fn generate_test(fn_list: &Vec<String>) -> String {

    format!("{}\n\nvoid main(void) {{\n{}\n}}\n", 
            format_and_concat(|label| format!("\nvoid {}(void);", label), fn_list),
            format_and_concat(|label| format!("\n\t{}();", label), &fn_list.clone()))
        .to_owned()
}

fn format_and_concat<F>(apply_format: F, fn_list: &Vec<String>) -> String
    where F : Fn(&String) -> String {

    fn_list.iter()
        .map(apply_format)
        .fold(String::new(), |a,b| format!("{}{}", a, b))
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
