extern crate rust_nlp100;

use std::collections::HashSet;
use std::collections::HashMap;
use rust_nlp100::tokenizer::Words;

fn main() {
    // inputs
    let s = "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can.";
    let mask = [1, 5, 6, 7, 8, 9, 15, 16, 19].iter()
        .map(|i| i - 1 as usize)
        .collect::<HashSet<_>>();

    let mut word2id = HashMap::new();
    for (i, w) in s.words().enumerate() {
        let n = if mask.contains(&i) { 1 } else { 2 };
        let word = w.chars().take(n).collect::<String>();
        word2id.insert(word, i);
    }
    println!("{:?}", word2id);
}
