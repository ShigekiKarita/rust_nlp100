pub trait Tokenize {
    fn tokenize(&self) -> Tokenizer;
}

impl Tokenize for str {
    fn tokenize(&self) -> Tokenizer {
        Tokenizer { siter: self.split_whitespace() }
    }
}

pub struct Tokenizer<'a> {
    siter: std::str::SplitWhitespace<'a>,
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.siter.next().map(
            |s| s
                .trim_end_matches(".")
                .trim_end_matches(","))
    }
}


#[test]
fn test_trim_end() {
    assert_eq!("he.llo", "he.llo.".trim_end_matches("."));
    assert_eq!("he.llo", "he.llo,".trim_end_matches(","));
}

#[test]
fn test_tokenize() {
    let s = "This is, well, tokenizer.";
    let mut t = s.tokenize();
    assert_eq!(Some("This"), t.next());
    assert_eq!(Some("is"), t.next());
    assert_eq!(Some("well"), t.next());
    assert_eq!(Some("tokenizer"), t.next());
    assert_eq!(None, t.next());
}
