fn main() {
    let s = String::from("Hello World");
    let res = find_word(&s);
    println!("{res}");
}

fn find_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
