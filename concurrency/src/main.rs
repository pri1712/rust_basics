use std::thread;

fn main() {

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
        }
    });
    thread::spawn(||{
        println!("Hello world part 2");
    });
    handle.join().unwrap();
}
