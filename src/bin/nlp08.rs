fn cipher(s: &str) -> String {
    s.chars().map(
        |c|
        if c.is_ascii_lowercase() { (219 - c as u8) as char }
        else {c}
    ).collect::<String>()
}

fn decipher(s: &str) -> String {
    s.chars().map(
        |e| {
            if 219 > e as u8 {
                let c = (219 - e as u8) as char;
                if c.is_ascii_lowercase() {
                    c
                } else {
                    e
                }
            }
            else {e}
        }
    ).collect::<String>()
}

fn main() {
    let s = "Hello 123";
    let c = cipher(s);
    let d = decipher(&c);
    println!("{}", s);
    println!("{}", c);
    println!("{}", d);
    assert_eq!(s, d);
    println!("{:?}", s.chars().map(|c| c as u8));
    println!("{:?}", decipher(&c).chars().map(|c| c as u8));
}
