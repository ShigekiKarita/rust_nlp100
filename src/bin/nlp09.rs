fn main() {
    let s = "I couldn't believe that I could actually understand what I was reading : the phenomenal power of the human mind .";
    println!("{}", s);

    let result = s.split_whitespace().map(
        |s|
        if s.len() > 4 {
            let mut cs = s.chars();
            let h = cs.next().unwrap();
            let mut m = cs.rev();
            let t = m.next().unwrap();
            let m: String = m.collect();
            format!("{}{}{}", h, m, t)
        }
        else {s.to_string()}
    ).collect::<Vec<_>>().join(" ");
    println!("{}", result);
}
