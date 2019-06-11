pub struct Tokenizer<'a> {
    siter: std::str::SplitWhitespace<'a>,
}

impl<'a> Tokenizer<'a> {
    pub fn from(s: &'a str) -> Tokenizer<'a> {
        Tokenizer { siter: s.split_whitespace() }
    }
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
