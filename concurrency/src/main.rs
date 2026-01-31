use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vec = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("clone"),
            String::from("thread"),
        ];
        for val in vec {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });

    thread::spawn(move || {
        let vec = vec![
            String::from("val1"),
            String::from("val2"),
            String::from("val3"),
            String::from("val4"),
        ];
        for val in vec {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

}
