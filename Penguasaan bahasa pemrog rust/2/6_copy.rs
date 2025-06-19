fn main() {
    let x = 5;
    let y = x; // copy, bukan move
    println!("x = {}, y = {}", x, y); // keduanya valid
}
