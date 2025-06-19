struct Marker;

fn print_marker(_: Marker) {
    println!("Marker struct dipanggil!");
}

fn main() {
    let m = Marker;
    print_marker(m);
}
