fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }

    largest
}

fn main() {
    let int_list = [1, 2, 3];
    let largest_int = largest(&int_list);

    let char_list = ['a', 'z', 'f'];
    let largest_char = largest(&char_list);

    println!("{largest_int}, {largest_char}");
}
