use regex;

pub fn parse_testfn_list(files: &Vec<String>) -> Vec<String> {
    let parser = Parser::new();

    files.iter()
        .map(|file| parser.parse_labels(file))
        .fold(Vec::new(), |mut a, b| { a.extend(b); a })
}

struct Parser {
    fn_label_regex: regex::Regex
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            fn_label_regex: Self::label_regex()
        }
    }

    fn label_regex() -> regex::Regex {
        regex!(r"(?x) #set ignore-whitespace mode
            ///             #c-style comment with extra slash
            .*              #any number of non-newlines
            \[test\].*      #test tag
            \s*void\s+      #require void return type
            (\w+)           #capture function identifier
            \s*\(\s*        #start of parameter list
            (\s*|void)      #match void or empty parameters
            \s*\)\s*        #end of parameter list
            \{.*            #require open brace (only match definitions)
            ")
    }

    pub fn parse_labels(&self, file: &String) -> Vec<String> {
        self.fn_label_regex.captures_iter(file)
            .map(|regex_match| String::from(regex_match.at(1).unwrap()))
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod test {
    trait ToOwnedStringVec {
        fn to_owned_vec(&self) -> Vec<String>;
    }

    impl ToOwnedStringVec for Vec<&'static str> {
        fn to_owned_vec(&self) -> Vec<String> {
            self.iter()
                .map(|&item| item.to_owned())
                .collect()
        }
    }

    #[test]
    fn good_fn() {
        assert_correct_labels_generated(
            vec![
                "test_function",
                "inner_scope",
                "brace_style",
                "no_close_delim",
                "void_arg"
            ],
            vec!["
                void some_function(void) {
                }

                void other_function () {
                }

                ///[test]
                void test_function() {
                }

                ///[test]
                void inner_scope() {
                    { }
                }

                ///[test]
                void brace_style()
                {
                }

                ///[test]
                void no_close_delim() {
                }

                ///[test]
                void void_arg(void) {
                }
            "]
            );
    }

    fn assert_correct_labels_generated(expected: Vec<&'static str>, actual: Vec<&'static str>) {
        let actual_labels = super::parse_testfn_list(&actual.to_owned_vec());
        let expected_labels = expected.to_owned_vec();

        assert_eq!(expected_labels.len(), actual_labels.len());
        for label in expected_labels {
            assert!(actual_labels.contains(&label));
        }

    }

    #[test]
    fn bad_fn() {
        assert_correct_labels_generated(
            vec![],
            vec!["
                void normal_fn() {
                }

                ///[test]
                int non_void() {
                }

                ///[test]
                void parameters(thing) {
                }
            "]
            );
    }

    #[test]
    fn multi_source() {
        assert_correct_labels_generated(
            vec![
                "source1_fn1",
                "source1_fn2",
                "source2_fn1",
                "source2_fn2"
            ],
            vec![
                "
                    ///[test]
                    void source1_fn1() {}
                    ///[test]
                    void source1_fn2() {}
                ",
                "
                    ///[test]
                    void source2_fn1() {}
                    ///[test]
                    void source2_fn2() {}
                "
            ]
            );
    }
}
 
