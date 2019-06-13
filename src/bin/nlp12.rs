/// Ex. 12 cut N-th column of /data/hightemp.txt, Ch. 2 UNIX command basics

#[derive(Debug)]
struct Config {
    filename: String,
    delim: char,
    ncol: usize
}

impl Config {
    fn new() -> Self {
        Self { filename: String::new(), delim: '\t', ncol: 0 }
    }

    fn parse(&mut self, it: &mut impl Iterator<Item = String>) -> &Self {
        // let mut it = args.iter();
        let arg0 = it.next().unwrap();
        self.filename = String::new();
        let usage = format!(r#"
cut the field of file

usage: {} -d <delim: String> -f <fields: usize> <filename: String>
"#, arg0);
        loop {
            match it.next() {
                Some(s) => match s.as_str() {
                    "-d" => {
                        let d = it.next().unwrap();
                        let mut c = d.chars();
                        self.delim = c.next().unwrap();
                        assert!(c.next() == None, "delimiter should be char but: {}", d);
                    },
                    "-f" => {
                        let n = it.next().unwrap().parse::<usize>().unwrap();
                        assert!(n > 0,
                                "arg <fields: usize> must be > 0 but {}", self.ncol);
                        self.ncol = n - 1;
                    }
                    _ => {
                        assert!(self.filename == "" &&
                                s.chars().next().unwrap() != '-',
                                "[ERROR] unexpected arg: {}\n\n{}", s, usage);
                        self.filename = s.to_string();
                    }
                },
                None => break
            }
        }
        self
    }

    fn parse_args() -> Self {
        let mut config = Self::new();
        config.parse(&mut std::env::args());
        config
    }
}

fn main() -> std::io::Result<()> {
    let config = Config::parse_args();
    use std::io::prelude::*;
    let f = std::fs::File::open(&config.filename)?;
    let f = std::io::BufReader::new(f);
    for rl in f.lines() {
        for l in rl {
            println!("{}", l.split(config.delim).nth(config.ncol).unwrap())
        }
    }
    Ok(())
}
