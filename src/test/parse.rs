
pub fn parse_testfn_list(files: &Vec<String>) -> Vec<String> {
    files.iter()
        .map(|file| parse_testfn_labels(file))
        .fold(Vec::new(), |mut a, b| { a.extend(b); a })
}

fn parse_testfn_labels(file: &String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
        
    let test_fn_regex = regex!(r"(?xm)
        ///             #c-style comment with extra slash
        .*              #any number of non-newlines
        \[test\].*      #test tag
        \s*void\s+      #require void return type
        (\w+)           #capture function identifier
        \s*\(\s*        #start of parameter list
        (\s*|void)      #match void or empty parameters
        \s*\)\s*        #end of parameter list
        \{.*            #require open brace (only match definitions)
        ");

    for regex_match in test_fn_regex.captures_iter(file) {
        println!("match {}", regex_match.at(0).unwrap());
        println!("cap {}", regex_match.at(1).unwrap());
        result.push(String::from(regex_match.at(1).unwrap()));
    }

    result
}

#[cfg(test)]
mod test {
    #[test]
    fn parse_testfn_list() {
        let source: Vec<String> = vec!["
            void some_function(void) {
            }

            void other_function () {
            }

            ///[test]
            void test_function() {
            }

            ///[test]
            void other_test_function (void) {
            }

            ///[test]
            void inner_scope() {
                does stuff;
                other stuff;

                if (something) {
                    more stuff;
                }

            }

            ///[test]
            void incorrect_brace_style()
            {
            }

            ///[test]
            void missing_close_delim() {

            ///[test]
            int bad_test_function() {
            }

            ///[test]
            void bad_args(thing) {
            }

           
            ".to_owned()];

        let actual_labels = super::parse_testfn_list(&source);

        let expected_labels = vec!["test_function".to_owned(),
                                   "other_test_function".to_owned(),
                                   "incorrect_brace_style".to_owned(),
                                   "inner_scope".to_owned(),
                                   "missing_close_delim".to_owned()];

        assert_eq!(expected_labels.len(), actual_labels.len());
        for label in expected_labels {
            assert!(actual_labels.contains(&label));
        }
    }
}
 
