fn main() {
    extern crate rust_nlp100;
    use rust_nlp100::tokenizer::Words;
    use rust_nlp100::ngram::NGram;

    let s = "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can.";

    println!("=== bi-gram chars ===");
    for ng in NGram::from(2, s.chars()) {
        println!("{:?}", ng);
    }

    println!("=== bi-gram words ===");
    for ng in NGram::from(2, s.words()) {
        println!("{:?}", ng);
    }

}
