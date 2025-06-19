fn main() {
    let s = String::from("rust");
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2); // OK: multiple immutable borrows
}
