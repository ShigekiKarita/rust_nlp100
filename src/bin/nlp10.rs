/// Ex. 10 Count lines of /data/hightemp.txt, Ch. 2 UNIX command basics


fn main() -> std::io::Result<()> {
    let args: Vec<_> = std::env::args().collect();
    let usage = format!("usage: {} <filename1> <filename2> ...", args[0]);
    if args.len() == 1 || args[1] == "--help" || args[1] == "-h" {
        println!("{}", usage);
    } else {
        let mut total = 0;
        let mut line_counts = vec![];
        let paths = &args[1..];
        for p in paths {
            use std::io::prelude::*;
            let f = std::fs::File::open(&p)?;
            let f = std::io::BufReader::new(f);
            let n = f.lines().count();
            line_counts.push(n);
            total += n;
        }
        let width = (total as f64).log10() as usize + 1;
        for (n, p) in line_counts.iter().zip(paths) {
            println!("{:width$} {}", n, p, width = width);
        }
        if args.len() > 2 {
            println!("{:width$} total", total, width = width)
        }
    }
    Ok(())
}
