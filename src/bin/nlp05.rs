extern crate rust_nlp100;

use rust_nlp100::tokenizer::Words;
use rust_nlp100::tokenizer::Tokenizer;

struct NGram<T: Iterator> {
    n: usize,
    tokn: T,
    prev: Vec<T::Item>,
    init: bool
}


impl<T: Iterator> NGram<T>
where <T as Iterator>::Item: Clone {
    fn from(n: usize, s: T) -> NGram<T> {
        NGram { n: n, tokn: s, prev: Vec::new(), init: false }
    }

    fn incomplete_next(&mut self) -> Option<Vec<T::Item>> {
        match self.tokn.next() {
            None => None,
            Some(s) => {
                self.prev.push(s);
                let l = self.prev.len();
                let s = if l < self.n { 0 } else { l - self.n };
                Some(self.prev[s .. l].to_vec())
            }
        }
    }
}

impl<T: Iterator> Iterator for NGram<T>
where <T as Iterator>::Item: Clone {
    type Item = Vec<T::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.init {
            for _ in 0 .. self.n - 1 {
                self.incomplete_next();
            }
            self.init = true
        }
        self.incomplete_next()
    }
}


fn main() {
    // inputs
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
