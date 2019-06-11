extern crate rust_nlp100;

fn main() {
    let a = "パトカー".to_string();
    let b = "タクシー".to_string();
    let mut c = String::new();
    for (ca, cb) in a.chars().zip(b.chars()) {
        c.push(ca);
        c.push(cb);
    }
    println!("{}", c);
}
