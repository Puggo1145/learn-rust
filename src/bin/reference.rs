fn main() {
    println!("1: Just Borrow");
    let s1 = String::from("Hello");
    let my_len = use_reference(&s1);
    println!("{my_len}\n");

    println!("2: Borrow as mutatable");
    let mut s2: String = String::from("Hello");
    mutate_with_reference(&mut s2);
    println!("{s2}\n");

    println!("3: Borrow as mutatable can only be used at once");
    let mut s3: String = String::from("Hello");
    // need to make sure no race between multiple borrow as mutable
    let borrow_mut_1: &mut String = &mut s3;
    mutate_with_reference(borrow_mut_1);
    println!("{s3}");
    // here, NLL automatically droped borrow_mut_1 since it is no longer used
    // so we won't see a panic below

    let borrow_mut_2: &mut String = &mut s3;        
    mutate_with_reference(borrow_mut_2);
    println!("{s3}");

    println!("4: Combine immutable and mutable borrow");
    // immutable and mutable borrow can also not be used at the same scope at the same time
    // but once in one of the scope, one of the two types of borrow do not overlap
    // we can use them
    let mut s4 = String::from("Ok");
    // immutable borrown
    let r1: &String = &s4;
    let r2: &String = &s4;
    println!("{r1} and {r2}");

    // compile knows that r1 and r2 won't be used below
    // so we can borrow as mutable
    let r3: &mut String = &mut s4;
    println!("{r3} \n");
}

fn use_reference(s: &String) -> usize {
    s.len()
}

fn mutate_with_reference(s: &mut String) {
    s.push_str(" world");
}
