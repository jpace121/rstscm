fn main() {
    let test = "(abc)".to_string();
    let token = tokenize(&test);
    
}

fn tokenize<'s>(chars: &'s String) -> Vec<&'s str> {
    let with_space: &'s String = chars.replace("(", "( ");
    let vected: Vec<&'s str>  = with_space.as_slice().split(' ').collect();
    return vected
}
