extern crate regex;
use self::regex::Regex;

pub fn parse_testfn_list(files: &Vec<String>) -> Vec<String> {
    let re = Regex::new(r"").unwrap();
    re.is_match("hi");
    Vec::new()
}
 
