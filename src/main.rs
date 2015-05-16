extern crate regex;
use std::collections::HashMap;
use regex::Regex;

enum SchmAtom {
    Int(i32),
    Float(f32),
    String(String),
    Symb(String),
    Vec(Vec<SchmAtom>),
    Func(Box<Fn(SchmAtom) -> SchmAtom>)
}

fn main() {

}

fn tokenize(input: String) -> Vec<String>{
    // Split input string into smaller tokens.
    let re1 = Regex::new(r"\(").unwrap();
    let re2 = Regex::new(r"\)").unwrap();
    let mut after: String = re1.replace_all(&input,"( ");
    after = re2.replace_all(&after, " )");

    let split = after.split(" "); //split is an iterator
    split.map(|x|{x.to_string()}).collect::<Vec<String>>()
    //^to `.to_string` seems kind of hackish...
    //BUT it actaully just forces the copy and move of the string
}

fn read_from_tokens(tokens : &mut Vec<String>)->SchmAtom{
    // Reads expressions from tokens.
    // Basically traight tolen from lisp.py

    let token : String = tokens.pop().unwrap(); //should try to reduce unwraps

    match token.as_ref() {
        "(" => {
            let mut l = vec!(); //uhh...
            loop{ //so dirty... (Is there a better way to recurse this?)
                match tokens[0].as_ref() {
                    ")" => break,
                     _ => l.push(read_from_tokens(tokens)) //<- need to unrecurse this
                }
            }
            tokens.pop(); //removes the ')' char
            return SchmAtom::Vec(l);
        },
         _  => return atom(token) 
    }
}

fn atom(token: String) -> SchmAtom {
    //Figure out what type a token is representing.
    
    let int_num = token.parse::<i32>();
    let float_num = token.parse::<f32>();

    if int_num.is_ok() {
        let x = int_num.unwrap();
        return SchmAtom::Int(x)
    } else if float_num.is_ok() {
        let x = float_num.unwrap();
        return SchmAtom::Float(x)
    } else { //must be string...
        match token.as_bytes()[0] as char {
            '\"' => return SchmAtom::String(token),
              _  => return SchmAtom::Symb(token)
        }
    }

}

fn parse(program : String) -> SchmAtom {
    let mut tokens: Vec<String> = tokenize(program);
    read_from_tokens(&mut tokens)
}

/*fn standard_env() -> HashMap<String,SchmAtom>{
    //Builds a standard scheme environment.
}*/

#[cfg(test)]

#[test]
fn test_atom(){
    let test_string = "\"ab\"".to_string();
    let test_float = "1.0".to_string();
    let test_int = "1".to_string();
    let test_symb = "hi".to_string();

    let string_atom = atom(test_string);
    let float_atom = atom(test_float);
    let int_atom = atom(test_int);
    let symb_atom = atom(test_symb);

    let res = match (string_atom, float_atom, int_atom, symb_atom) {
        (SchmAtom::String(x), SchmAtom::Float(y), SchmAtom::Int(z), 
                SchmAtom::Symb(l),) 
            => { (x,y,z,l) == ("\"ab\"".to_string(),1.0,1,"hi".to_string())},
          _ => false 
    };

    assert!(res,true)

}

#[test]
fn test_tokenize() {
    let test_input: String  = "(a b c)".to_string();
    assert_eq!(tokenize(test_input), 
               vec!("(","a","b","c", ")"));
}

