fn f(x: i32, y: &str, z: f32) -> String {
    format!("{}時の{}は{}", x, y, z)
}

fn main() {
    println!("{}", f(12, "気温", 22.4));
}
