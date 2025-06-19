fn main() {
    let s1 = String::from("clone me");
    let s2 = s1.clone(); // deep copy
    println!("{} and {}", s1, s2); // s1 tetap bisa digunakan
}
