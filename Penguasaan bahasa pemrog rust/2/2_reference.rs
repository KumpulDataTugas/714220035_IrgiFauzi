fn main() {
    let s1 = String::from("Rust");
    let len = get_length(&s1);

    println!("String: {}", s1);
    println!("Length: {}", len);
}

fn get_length(s: &String) -> usize {
    s.len()
}
