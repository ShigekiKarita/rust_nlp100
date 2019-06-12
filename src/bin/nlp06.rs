fn main() {
    use std::collections::HashSet;

    extern crate rust_nlp100;
    use rust_nlp100::ngram::NGram;

    let a = "paraparaparadise";
    let b = "paragraph";

    let x = a.chars().ngram(2).collect::<HashSet<_>>();
    let y = b.chars().ngram(2).collect::<HashSet<_>>();
    println!("union: {:?}", x.union(&y));
    println!("intersection: {:?}", x.intersection(&y));
}
