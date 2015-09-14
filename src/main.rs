#![deny(warnings)]
#![feature(plugin)]
#![plugin(regex_macros)]

extern crate regex;
extern crate term;
extern crate binunit;

use std::io::prelude::*;

fn main() {

    let binunit = binunit::BinUnit::new(&std::env::current_dir().unwrap());
    match binunit.run() {
        Ok(output) => 
            print(&output)
        ,
        Err(err) => 
            println!("{}\nbinunit failed", err)
    }

}

pub fn print(output: &Vec<String>) {

    let summary = output.iter()
        .inspect(|result| print_result_colorized(&result.trim().to_owned()))
        .fold((0, 0, 0), |a, b| increment_summary(a, b));

    print_summary(&summary);
}

fn print_result_colorized(output: &String) {

    let mut term = term::stdout().unwrap();
    for capture in regex!(r"(?m)(.*: )(failed|ok)(.*)|.*").captures_iter(output) {
        match capture.at(2).unwrap_or("") {
            "ok" => {
                write!(term, "{}", capture.at(1).unwrap()).unwrap();
                term.fg(term::color::BRIGHT_GREEN).unwrap();
                write!(term, "{}", capture.at(2).unwrap()).unwrap();
                term.reset().unwrap();
                writeln!(term, "{}", capture.at(3).unwrap()).unwrap();
            },
            "failed" => {
                write!(term, "{}", capture.at(1).unwrap()).unwrap();
                term.fg(term::color::BRIGHT_RED).unwrap();
                write!(term, "{}", capture.at(2).unwrap()).unwrap();
                term.reset().unwrap();
                writeln!(term, "{}", capture.at(3).unwrap()).unwrap();
            },
            _ => writeln!(term, "{}\n", capture.at(0).unwrap()).unwrap()
        }
    }
}

fn increment_summary((total, pass, fail): (i32, i32, i32), result: &String) -> (i32, i32, i32) {

    if result.contains(": ok") {
        (total+1, pass+1, fail)
    } else if result.contains(": failed") {
        (total+1, pass, fail+1)
    } else {
        (total, pass, fail)
    }
}

fn print_summary(&(_, pass, fail): &(i32, i32, i32)) {

    let mut term = term::stdout().unwrap();
    write!(term, "\n\nsummary: ").unwrap();
    match fail { 
        0 => {
            term.fg(term::color::BRIGHT_GREEN).unwrap();
            write!(term, "ok").unwrap()
        },
        _ => {
            term.fg(term::color::BRIGHT_RED).unwrap();
            write!(term, "failed").unwrap();
        }
    }
    term.reset().unwrap();
    writeln!(term, " -- {} passed, {} failed\n\n", pass, fail).unwrap();
}
