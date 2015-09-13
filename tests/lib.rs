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

    let output = binunit.run().unwrap();

    assert_all_labels_match(&output);
    assert_output_does_not_link(&output);
}

fn assert_output_does_not_link(output: &String) {

    let lines = output.trim().split("\n");
    for line in lines {
        assert!(line.contains("could not link"));
        assert!(!line.contains("ok"));
        assert!(!line.contains("failed"));
        assert!(!line.contains("segfaulted"));
    }
}

fn assert_all_labels_match(output: &String) {

    println!("output:\n{}", output);

    let labels_from_file = read_test_labels();

    let lines = output.trim().split("\n");
    assert_eq!(labels_from_file.len(), lines.clone().count());

    for label in labels_from_file {
        let regex = regex::Regex::new(&format!("(?m)^{}:.*$", label)).unwrap();
        assert!(regex.is_match(&output));
        assert_eq!(1, regex.captures_iter(&output).count());
    }
}

#[test]
fn testc_compile() {

    fs::create_dir("./tests/testc-build").unwrap_or(());
    Command::new("gcc")
        .current_dir("./tests")
        .arg("testc/passfail.c")
        .arg("-c")
        .arg("-o")
        .arg("testc-build/passfail.o")
        .status().unwrap();

    let binunit = binunit::BinUnit::new(&PathBuf::from("./tests"));
    let output = binunit.run().unwrap();

    assert_all_labels_match(&output);
    assert_output_satisfies(&output);
}

fn assert_output_satisfies(output: &String) {

    let regex = regex!(r"(?m)^(.*):\s*(.*)\s*$");
    for cap in regex.captures_iter(output) {
        let label = cap.at(1).unwrap();
        let test_result = cap.at(2).unwrap();

        match test_result {
            "ok" => assert!(label.contains("pass")),
            "failed" => assert!(label.contains("fail")),
            "failed (segfault)" => assert!(label.contains("segf")),
            _ => panic!("unrecognized test result at line: {}", test_result)
        }
    }

}
