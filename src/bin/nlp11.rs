/// Ex. 11 replace tab to space /data/hightemp.txt, Ch. 2 UNIX command basics


fn main() -> std::io::Result<()> {
    let args: Vec<_> = std::env::args().collect();
    let usage = format!("usage: {} <filename>", args[0]);
    if args.len() != 2 || args[1] == "--help" || args[1] == "-h" {
        println!("{}", usage);
    } else {
        let p = &args[1];
        use std::io::prelude::*;
        let f = std::fs::File::open(&p)?;
        let f = std::io::BufReader::new(f);
        for l in f.lines() {
            println!("{}", l.unwrap().replace("\t", " "));
        }
    }
    Ok(())
}
