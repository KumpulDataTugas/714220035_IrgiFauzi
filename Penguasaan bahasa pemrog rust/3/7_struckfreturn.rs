struct Point {
    x: i32,
    y: i32,
}

fn origin() -> Point {
    Point { x: 0, y: 0 }
}

fn main() {
    let p = origin();
    println!("Origin: ({}, {})", p.x, p.y);
}
