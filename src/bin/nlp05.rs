fn main() {
    extern crate rust_nlp100;
    use rust_nlp100::tokenizer::Words;
    use rust_nlp100::ngram::NGram;

    let s = "I am an NLPer";

    println!("=== bi-gram chars ===");
    for ng in s.chars().ngram(2) {
        println!("{:?}", ng);
    }

    println!("=== bi-gram words ===");
    for ng in s.words().ngram(2) {
        println!("{:?}", ng);
    }

}
