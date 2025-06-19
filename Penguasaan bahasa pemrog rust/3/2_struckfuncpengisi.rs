struct User {
    name: String,
    age: u32,
}

fn create_user(name: String, age: u32) -> User {
    User { name, age }
}

fn main() {
    let user = create_user(String::from("Bob"), 25);
    println!("User: {} ({})", user.name, user.age);
}
