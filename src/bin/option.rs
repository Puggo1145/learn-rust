fn add(a: u32, b: Option<u32>) -> Result<u32, bool> {
    match b {
        Some(b) => Ok(a + b),
        None => Err(false)
    }
}

fn check_val(val: Option<u32>) -> Option<u32> {
    let Some(state) = val else {
        println!("Value not exists");
        return None;
    };

    Some(state)
}

fn main() {
    let b: Option<u32> = Some(5);
    let res = add(4, b);
    println!("{:?}", res);

    let some_val = Some(1);
    if let Some(val) = some_val {
        println!("{val}");
    } else {
        println!("nothing")
    }

    let res = check_val(Option::None);
    println!("{res:?}")
}