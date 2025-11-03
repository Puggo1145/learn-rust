fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let v1_map_iter = v1_iter.map(|x| x + 1);
    for val in v1_map_iter {
        println!("Mapped to {val}");
    }
}
