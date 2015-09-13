#![deny(warnings)]

use std::path::PathBuf;
use std::process::Command;
use std::fs;

extern crate binunit;


#[test]
fn testc_no_compile() {

    let binunit = binunit::BinUnit::new(&PathBuf::from("./tests/testc"));

    let output = binunit.run().unwrap();

    println!("output:\n{}", output);
    assert!(output.contains("test_pass: could not link"));
    assert!(output.contains("test_fail: could not link"));
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

    println!("output:\n{}", output);
    assert!(output.contains("test_pass: ok"));
    assert!(output.contains("test_fail: failed"));
}
