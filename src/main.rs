#![feature(core)] //this should be removed before out of beta
extern crate regex;
use regex::Regex;

enum schmAtom {
    Int(i32),
    Float(f32),
    String(String),
    Symb(String),
    Vec(Vec<schmAtom>)
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

fn read_from_tokens(mut tokens : Vec<String>)->schmAtom{
    // Reads expressions from tokens.
    // Basically traight tolen from lisp.py

    let token = tokens.pop().unwrap(); //should try to reduce unwraps

    match token.as_slice() {
        "(" => {
            let mut L = vec!();
            loop{ //so dirty... (Is there a better way to recurse this?)
                match tokens[0].as_slice() {
                    ")" => break,
                     _ => L.push(read_from_tokens(tokens)) //<- need to unrecurse this
                }
            }
            tokens.pop(); //removes the ')' char
            return schmAtom::Vec(L);
        },
         _  => return atom(token) 
    }
}

fn atom(token: String) -> schmAtom {
    //Figure out what type a token is representing.
    
    let intNum = token.parse::<i32>();
    let floatNum = token.parse::<f32>();

    if intNum.is_ok() {
        let x = intNum.unwrap();
        return schmAtom::Int(x)
    } else if floatNum.is_ok() {
        let x = floatNum.unwrap();
        return schmAtom::Float(x)
    } else { //must be string...
        match token.as_bytes()[0] as char {
            '\"' => return schmAtom::String(token),
              _  => return schmAtom::Symb(token)
        }
    }

}


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
        (schmAtom::String(x), schmAtom::Float(y), schmAtom::Int(z), 
                schmAtom::Symb(l),) 
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



