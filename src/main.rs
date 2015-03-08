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

fn tokenize(input: & String) -> Vec<&str>{
    //let mut tokens = vec![];
    
    let re1 = Regex::new(r"\(").unwrap();
    let re2 = Regex::new(r"\)").unwrap();
    /* This doesn't compile because `after` only lives in the function, 
       but we keep using it's adress all over the place. 
       What I don't see is how to make after "live" longer.
       In C, I would allocate to heap, and manually manage lifetime
       from there. Alternatively, I would pass in something with lifetime
       I wanted and fill that memory. Or maybe try a Copy somewhere?

       Maybe I need to decalre after as an Rc?
     */
    let mut after: String = re1.replace_all(input,"( ");
    after = re2.replace_all(&after, " )");

    let split = after.split(" "); //split is an iterator
    split.collect::<Vec<&str>>()

    //I bet there is a better way to do this, but I keep getting lifetime errors.
    /*for token in split {
        tokens.push(token);
    }*/

    //tokens
    
}

fn read_from_tokens(){
}


