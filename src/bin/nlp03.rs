extern crate rust_nlp100;

fn main() {
    let s = String::from("Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.");
    for raw in s.split_whitespace() {
        let mut w = raw.to_string();
        w.retain(|c| c != '.' && c != ',');
        println!("{}: {}", w, w.len());
    }
}
