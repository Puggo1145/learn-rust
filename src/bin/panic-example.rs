use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let res = File::open("test.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("test.txt").unwrap_or_else(|error| {
                panic!("Even cannot create file because: {error}");
            })
        } else {
            panic!("{error}");
        }
    });
}
