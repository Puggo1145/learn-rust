use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<&str, i32> = HashMap::new();
    scores.insert("Blue", 10);
    let s = scores.get("Blue").copied().unwrap_or(0);
    println!("{s}");
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    scores.insert("Blue", 23);
    let s = scores.get("Blue").copied().unwrap_or(0);
    println!("{s}");

    scores.entry("Yellow").or_insert(20);
    scores.entry("Blue").or_insert(20);
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}");
}
