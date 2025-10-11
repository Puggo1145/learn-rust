fn main() {
    let s = String::from("Hello");
    let my_len = use_reference(&s);
    println!("{my_len}");
}

fn use_reference(s: &String) -> usize {
    s.len()
}
