extern crate rust_nlp100;

fn main() {
    let s = "パタトクカシーー";
    let word = [0, 2, 4, 6].iter().map(|&i| s.chars().nth(i).unwrap()).collect::<String>();
    println!("{}", word);
}
