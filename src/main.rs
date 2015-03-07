extern crate regex;
use regex::Regex;

fn main() {

    let test = "(abc)".to_string();
    let tokened: String = tokenize(&test);
    
    println!("{}",tokened)
}

fn tokenize(input: &String) -> String{
    
    let re1 = Regex::new(r"\(").unwrap();
    let re2 = Regex::new(r"\)").unwrap();
    let after = re1.replace_all(input,"( ");
    re2.replace_all(after.as_slice(), " )")
}
