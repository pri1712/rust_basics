use std::{io};

fn main() {
    println!("Enter the value of N");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: u32 = n.trim().parse().expect("Please type a number!");
    let result = fibonacci(n);
    println!("{}", result);
}

fn fibonacci(n: u32) -> u32 {
    let mut cur = 1;
    let mut prev = 0;
    for _ in 1..n-1 {
        let temp = cur;
        cur+=prev;
        prev = temp;
    }
    cur
}
