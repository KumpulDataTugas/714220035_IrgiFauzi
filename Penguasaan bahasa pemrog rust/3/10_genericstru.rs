struct Pair<T> {
    a: T,
    b: T,
}

impl<T: std::fmt::Display> Pair<T> {
    fn print(&self) {
        println!("Pair: {}, {}", self.a, self.b);
    }
}

fn main() {
    let pair = Pair { a: 10, b: 20 };
    pair.print();
}
