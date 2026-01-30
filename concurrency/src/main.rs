use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn main() {

    thread::spawn(||{
        println!("Hello, world!");
    });
    sleep(Duration::from_millis(100));
    thread::spawn(||{
        println!("Hello world part 2");
    });

}
