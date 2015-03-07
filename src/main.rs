extern crate regex;
use regex::Regex;

fn main() {
    let re1 = Regex::new(r"\(").unwrap();
    let re2 = Regex::new(r"\)").unwrap();

    let test = "(abc)";
    
    let mut after = re1.replace_all(test,"( ");
    after = re2.replace_all(after.as_slice(), " )");

    println!("{}",after)
}

/*
fn tokenize<'s>(chars: &'s String) -> Vec<&'s str> {
    let with_space: &'s String = chars.replace("(", "( ");
    let vected: Vec<&'s str>  = with_space.as_slice().split(' ').collect();
    return vected
}
*/
