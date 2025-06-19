fn main() {
    {
        let s = String::from("scoped");
        println!("{}", s);
    }
    // println!("{}", s); // Error: s sudah di luar scope
}
