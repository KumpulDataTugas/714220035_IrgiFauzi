fn main() {
    let s1 = String::from("Ownership Example");
    let s2 = s1; // Ownership pindah ke s2

    // println!("{}", s1); // Ini akan error karena s1 sudah tidak memiliki ownership
    println!("s2: {}", s2);
}
