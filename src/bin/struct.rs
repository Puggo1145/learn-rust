struct User {
    username: String,
    email: String,
    age: i32,
    active: bool
}

fn create_user(username: String, email: String, age: i32) -> User {
    User {
        username,
        email,
        age,
        active: true,
    }
}

struct Color(i32, i32, i32);

fn main() {
    println!("1. normal struct use");
    let mut user1 = create_user(
        String::from("Andrew"),
        String::from("123@mail.com"), 
        22
    );
    let user2 = User {
        username: String::from("Alan"),
        age: 23,
        ..user1
    };
    println!("{0}", user2.email);
    // for heap data, the ownership is moved to user2
    // the user1 struct is partially invalid status: email is moved
    user1.email = String::from("changed");
    println!("{0}, {1}", user2.email, user1.email);
    // for stack data, the ownership is copied to user2
    user1.active = false;
    println!("{0}", user1.active);

    println!("2. tuple struct");
    let black = Color(0, 0, 0);
    let Color(r, g, b) = black;
    println!("{r}, {g}, {b}");
}
