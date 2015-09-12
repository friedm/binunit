
/// Generate a source file for the test executable
/// from a list of test function names.
pub fn generate_test(fn_list: &Vec<String>) -> String {
    let mut call_list = String::new();

    for fn_id in fn_list {
        call_list = format!("{}\n\t{}();", call_list, fn_id);
    }

    return format!("void main(void) {{
    {}
}}\n", call_list).to_owned();
}

#[cfg(test)]
mod test {
    use ToOwnedStringVec;

    #[test]
    fn test() {
        let fn_list = vec!["fn1", "fn2"].to_owned_vec();

        let actual = super::generate_test(&fn_list);
        println!("generated: {}", actual);

        assert!(actual.contains("void main(void)"));

        for fn_id in fn_list.iter() {
            assert!(actual.contains(&format!("{}();", fn_id)[..]));
        }
    }
}
