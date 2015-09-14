
/// Generate a source file for the test executable
/// from a list of test function names.
pub fn generate_test(fn_list: &Vec<String>) -> String {

    format!("{}{}\n\nvoid binunit_run_test_with_label(char *label) {{\n{}\n}}\n", 
            "#include \"binunit.h\"\n#include <string.h>",
            fn_list.map_and_concat(|label| format!("\nextern void {}(void) __attribute__((weak));", label)),
            fn_list.map_and_concat(|label| format!("\n\tif (0 == strcmp(label, \"{0}\")) binunit_run_test({0}, \"{0}\");", label)))
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

        assert_generated_contains("void binunit_run_test_with_label(char *label)", test_fn_list());
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
            assert_generated_contains(&format!("if (0 == strcmp(label, \"{0}\")) binunit_run_test({0}, \"{0}\");", fn_id)[..], test_fn_list());
        }
    }

    #[test]
    fn define_test() {

        for fn_id in test_fn_list() {
            assert_generated_contains(&format!("extern void {}(void) __attribute__((weak));", fn_id)[..], test_fn_list());
        }
    }

    #[test]
    fn include_test() {

        assert_generated_contains("#include <string.h>\n", test_fn_list());
        assert_generated_contains("#include \"binunit.h\"\n", test_fn_list());
    }
}
