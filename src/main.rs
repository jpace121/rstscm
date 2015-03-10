#![feature(core)] //this should be removed before out of beta
extern crate regex;
use regex::Regex;

enum schmAtom {
    Int(i32),
    Float(f32),
    Strng(String),
    Symb(String)
}

fn main() {

    let test = "(abc)".to_string();
    let tokened = tokenize(&test);
    
    println!("{:?}",tokened)
}

fn tokenize(input: & String) -> Vec<String>{
    //let mut tokens = vec![];
    
    let re1 = Regex::new(r"\(").unwrap();
    let re2 = Regex::new(r"\)").unwrap();
    let mut after: String = re1.replace_all(input,"( ");
    after = re2.replace_all(&after, " )");

    let split = after.split(" "); //split is an iterator
    split.map(|x|{x.to_string()}).collect::<Vec<String>>()
    //^to `.to_string` seems kind of hackish...
}

fn read_from_tokens(){
}


