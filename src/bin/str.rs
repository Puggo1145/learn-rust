fn main() {
    let s1 = String::from("áh");
    // c is a unicode scalar
    for c in s1.chars() {
        println!("{c}")
    }
    // b is real byte
    for b in s1.bytes() {
        println!("{b}")
    }
}
