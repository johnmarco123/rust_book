fn get_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s;
        }
    }
    &s[..]
}

fn main() {
    let s = String::from("hello world");
    let word = get_first_word(&s);
    print!("{}", word);
}
