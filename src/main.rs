fn main() {
    let test = "(abc)";
    let token = tokenize(test.to_string());
    println!("{}",token)
}

fn tokenize(chars: String) -> Vec<String> {
    let with_sapace = chars.replace("(", "( ");
    chars.as_slice().split(' ').collect()
}
