struct Point<T> {
    x: T,
    y: T,
}

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
    println!("1. Function generics");

    let int_list = [1, 2, 3];
    let largest_int = largest(&int_list);

    let char_list = ['a', 'z', 'f'];
    let largest_char = largest(&char_list);

    println!("{largest_int}, {largest_char}\n");

    println!("2. Struct generics");
    let int_point = Point {x: 1, y: 2};
    let float_point = Point {x: 1.9, y: 2.1};
    println!("{}, {}", int_point.x, float_point.y)
}
