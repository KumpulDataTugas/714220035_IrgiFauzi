struct Circle {
    radius: f64,
}

impl Circle {
    fn new(radius: f64) -> Self {
        Self { radius }
    }
}

fn main() {
    let c = Circle::new(4.0);
    println!("Radius: {}", c.radius);
}
