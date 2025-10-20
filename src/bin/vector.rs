#[derive(Debug)]
enum MultipleTypes {
    Int(i32),
    Float(f32),
    Text(String),
}

fn main() {
    let multiple_types = vec![
        MultipleTypes::Int(3),
        MultipleTypes::Float(3.14),
        MultipleTypes::Text(String::from("Hello World")),
    ];
    for item in &multiple_types {
        println!("{item:?}");
    }
}