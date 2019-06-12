use std::collections::VecDeque;

pub struct NGramIter<T: Iterator> {
    n: usize,
    tokn: T,
    prev: VecDeque<T::Item>,
    init: bool
}


impl<T: Iterator> NGramIter<T>
where <T as Iterator>::Item: Clone {
    pub fn from(s: T, n: usize) -> NGramIter<T> {
        NGramIter { n: n, tokn: s, prev: VecDeque::new(), init: false }
    }

    pub fn incomplete_next(&mut self) -> Option<Vec<T::Item>> {
        self.tokn.next().map(|s| {
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
            v
        })
    }
}

impl<T: Iterator> Iterator for NGramIter<T>
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

pub trait NGram<T: Iterator>
where <T as Iterator>::Item: Clone {
    fn ngram(self, n: usize) -> NGramIter<T>;
}

impl<T: Iterator> NGram<T> for T
where <T as Iterator>::Item: Clone {
    fn ngram(self, n: usize) -> NGramIter<T> {
        NGramIter::from(self, n)
    }
}

#[test]
fn test_char_bigram() {
    let s  = "abc";
    let mut ngram = s.chars().ngram(2);
    assert_eq!(Some(vec!['a', 'b']), ngram.next());
    assert_eq!(Some(vec!['b', 'c']), ngram.next());
    assert_eq!(None, ngram.next());
}

#[test]
fn test_char_trigram() {
    let s  = "abcd";
    let mut ngram = s.chars().ngram(3);
    assert_eq!(Some(vec!['a', 'b', 'c']), ngram.next());
    assert_eq!(Some(vec!['b', 'c', 'd']), ngram.next());
    assert_eq!(None, ngram.next());
}

#[test]
fn test_word_bigram() {
    use super::tokenizer::Words;
    let s  = "abc, d e.";
    let mut ngram = s.words().ngram(2);
    assert_eq!(Some(vec!["abc", "d"]), ngram.next());
    assert_eq!(Some(vec!["d", "e"]), ngram.next());
    assert_eq!(None, ngram.next());
}

#[test]
fn test_word_tiigram() {
    use super::tokenizer::Words;
    let s  = "abc, d e abc.";
    let mut ngram = s.words().ngram(3);
    assert_eq!(Some(vec!["abc", "d", "e"]), ngram.next());
    assert_eq!(Some(vec!["d", "e", "abc"]), ngram.next());
    assert_eq!(None, ngram.next());
}

