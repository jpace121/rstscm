extern crate regex;
use regex::Regex;

fn main() {

    let test = "(abc)".to_string();
    let tokened: Vec<String> = tokenize(&test);
    
    println!("{:?}",tokened)
}

fn tokenize<'s>(input: &'s String) -> Vec<String>{
    let mut tokens = vec![];
    
    let re1 = Regex::new(r"\(").unwrap();
    let re2 = Regex::new(r"\)").unwrap();
    let mut after = re1.replace_all(input,"( ");
    after = re2.replace_all(after.as_slice(), " )");

    let after_sliced: &str = after.as_slice();
    let mut split = after_sliced.split_str(" ");

    //I bet there is a better way to do this, but I keep getting lifetime errors.
    for token in split {
        tokens.push(token.to_string());
    }

    tokens
}
