use std::collections::VecDeque;

pub struct NGram<T: Iterator> {
    n: usize,
    tokn: T,
    prev: VecDeque<T::Item>,
    init: bool
}


impl<T: Iterator> NGram<T>
where <T as Iterator>::Item: Clone {
    pub fn from(n: usize, s: T) -> NGram<T> {
        NGram { n: n, tokn: s, prev: VecDeque::new(), init: false }
    }

    pub fn incomplete_next(&mut self) -> Option<Vec<T::Item>> {
        match self.tokn.next() {
            None => None,
            Some(s) => {
                self.prev.push_back(s);
                while self.prev.len() > self.n {
                    self.prev.pop_front();
                }
                let l = self.prev.len();
                let s = if l < self.n { 0 } else { l - self.n };
                let mut v = Vec::with_capacity(self.n);
                for i in s .. l {
                    v.push(self.prev[i].clone());
                }
                Some(v)
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


#[test]
fn test_char_bigram() {
    let s  = "abc";
    let mut ngram = NGram::from(2, s.chars());
    assert_eq!(Some(vec!['a', 'b']), ngram.next());
    assert_eq!(Some(vec!['b', 'c']), ngram.next());
    assert_eq!(None, ngram.next());
}

#[test]
fn test_char_trigram() {
    let s  = "abcd";
    let mut ngram = NGram::from(3, s.chars());
    assert_eq!(Some(vec!['a', 'b', 'c']), ngram.next());
    assert_eq!(Some(vec!['b', 'c', 'd']), ngram.next());
    assert_eq!(None, ngram.next());
}

#[test]
fn test_word_bigram() {
    use super::tokenizer::Words;
    let s  = "abc, d e.";
    let mut ngram = NGram::from(2, s.words());
    assert_eq!(Some(vec!["abc", "d"]), ngram.next());
    assert_eq!(Some(vec!["d", "e"]), ngram.next());
    assert_eq!(None, ngram.next());
}

#[test]
fn test_word_tiigram() {
    use super::tokenizer::Words;
    let s  = "abc, d e abc.";
    let mut ngram = NGram::from(3, s.words());
    assert_eq!(Some(vec!["abc", "d", "e"]), ngram.next());
    assert_eq!(Some(vec!["d", "e", "abc"]), ngram.next());
    assert_eq!(None, ngram.next());
}

