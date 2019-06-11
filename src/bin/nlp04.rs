extern crate rust_nlp100;

use rust_nlp100::tokenizer::Tokenizer;

fn main() {
    let s = "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can.";
    // let mask = vec![1, 5, 6, 7, 8, 9, 15, 16, 19];
    assert_eq!("he.llo", "he.llo.".trim_end_matches("."));
    assert_eq!("he.llo", "he.llo,".trim_end_matches(","));

    for w in Tokenizer::from(s) {
        println!("{}: {}", w, w.len());
    }
}
