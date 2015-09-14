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
        Ok(mut output) => 
            print(&mut output)
        ,
        Err(err) => 
            println!("{}\nbinunit failed", err)
    }

}

pub fn print(output: &mut Vec<String>) {

    let summary = output.iter()
        .fold((0, 0, 0), |a, b| increment_summary(a, b));

    output.sort();

    println!("");

    for item in output.iter().filter(|item| item.contains(": ok")) {
        print_result_colorized(&item.trim().to_owned());
    }

    println!("\n");

    for item in output.iter().filter(|item| item.contains(": failed")) {
        print_result_colorized(&item.trim().to_owned());
    }

    print_summary(&summary);
}

fn print_result_colorized(output: &String) {

    let mut term = term::stdout().unwrap();
    for capture in regex!(r"(?m)(.*: )(ok|failed)(.*)|.*").captures_iter(output) {
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
    write!(term, "\n\n\t").unwrap();
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
    writeln!(term, " -- {} passed, {} failed\n", pass, fail).unwrap();
}
