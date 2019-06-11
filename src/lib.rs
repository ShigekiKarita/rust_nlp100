pub mod nlp100 {
    pub fn test() {
        println!("lib nlp100");
    }
}

pub mod tokenizer;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
