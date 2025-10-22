fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

struct ImportantExcept<'a> {
    part: &'a str,
}

fn main() {
    println!("1. Lifetime Annotations in Function Signatures");
    let result;
    let s1 = String::from("hello");
    let s2 = String::from("world!");
    result = longest(&s1, &s2);
    println!("{}", result);

    println!("2. Lifetime Annotations in Struct Definitions");
    let novel = String::from("Call me. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcept {
        part: first_sentence
    };
}
