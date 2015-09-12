
/// Generate a source file for the test executable
/// from a list of test function names.
pub fn generate_test(fn_list: &Vec<String>) -> String {
    fn_list.first().unwrap().to_owned()
}
