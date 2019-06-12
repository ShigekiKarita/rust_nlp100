fn main() {
    use std::collections::HashSet;

    extern crate rust_nlp100;
    use rust_nlp100::ngram::NGram;

    let a = "paraparaparadise";
    let b = "paragraph";

    let x: HashSet<_> = a.chars().ngram(2).collect();
    let y: HashSet<_> = b.chars().ngram(2).collect();
    println!("x ∪ y = {:?}", x.union(&y));
    println!("x ∩ y = {:?}", x.intersection(&y));

    let se = "se".chars().ngram(2).next().unwrap();
    println!("{:?} ∈ x = {}", se, x.contains(&se));
    println!("{:?} ∈ y = {}", se, y.contains(&se));
}
