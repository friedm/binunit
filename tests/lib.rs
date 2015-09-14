#![deny(warnings)]
#![feature(plugin)]
#![plugin(regex_macros)]

use std::path::PathBuf;
use std::process::Command;
use std::fs;
use std::io::prelude::*;

extern crate binunit;
extern crate regex;

fn read_test_labels() -> Vec<String> {

    let mut file = fs::File::open("./tests/labels.test").unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    let split = s.trim().split("\n");
    split.map(|label| label.to_owned())
        .collect::<Vec<_>>()
}

#[test]
fn testc_no_compile() {

    let binunit = binunit::BinUnit::new(&PathBuf::from("./tests/testc"));

    let output = binunit.run().unwrap()
        .iter()
        .fold(String::new(), |a,b| format!("{}{}", a, b));

    assert_all_labels_match(&output);
    assert_output_does_not_link(&output);
}

fn assert_output_does_not_link(output: &String) {

    let lines = output.trim().split("\n");
    for line in lines {
        assert!(line.contains("failed (could not link)"));
        assert!(!line.contains("ok"));
        assert!(!line.contains("failed (segfault)"));
    }
}

fn assert_all_labels_match(output: &String) {

    println!("output:\n{}", output);

    let labels_from_file = read_test_labels();

    for label in labels_from_file {
        println!(">{}", label);
        let regex = regex::Regex::new(&format!("(?m)^{}:.*$", label)).unwrap();
        assert!(regex.is_match(&output));
        assert_eq!(1, regex.captures_iter(&output).count());
    }
}

#[test]
fn testc_compile() {

    fs::create_dir("./tests/testc-build").unwrap_or(());
    compile("passfail");
    compile("assert");
    compile("assert_string");
    compile("assert_mem");
    compile("segf");

    let binunit = binunit::BinUnit::new(&PathBuf::from("./tests"));
    let output = binunit.run().unwrap()
        .iter()
        .fold(String::new(), |a,b| format!("{}{}", a, b));

    assert_all_labels_match(&output);
    assert_output_satisfies(&output);
}

fn compile(test_name: &str) {

     Command::new("gcc")
        .current_dir("./tests")
        .arg(&format!("testc/{}.c", test_name)[..])
        .arg("-c")
        .arg("-o")
        .arg(&format!("testc-build/{}.o", test_name)[..])
        .status().unwrap();
}

fn assert_output_satisfies(output: &String) {

    let regex = regex!(r"(?m)^(.*):\s*(.*)\s*$");
    for cap in regex.captures_iter(output) {
        let line = cap.at(0).unwrap().clone();
        let label = cap.at(1).unwrap();
        let test_result = cap.at(2).unwrap();

        println!(">{}", line);
        match test_result {
            "ok" => assert!(label.contains("pass")),
            "failed" => assert!(label.contains("fail")),
            "failed (segfault)" => assert!(label.contains("segf")),
            _ => panic!("unrecognized test result at line: {}", line)
        }
    }
}
