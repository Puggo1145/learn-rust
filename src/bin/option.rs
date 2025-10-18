fn add(a: u32, b: Option<u32>) -> Result<u32, bool> {
    match b {
        Some(b) => Ok(a + b),
        None => Err(false)
    }
}

fn main() {
    let b: Option<u32> = Some(5);
    let res = add(4, b);
    println!("{:?}", res);
}