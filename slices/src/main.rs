use std::io;
fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let ret = slice(&s);
    println!("{}", ret);
}
fn slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i,&iter) in bytes.iter().enumerate() {
        if iter == b'e' {
            return &s[0..i+1];
        }
    }
    &s[..]
}
