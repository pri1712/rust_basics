use std::thread;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;

fn main() {
   let m = Arc::new(Mutex::new(0));
   let mut handles = vec![];
   for i in 1..11 {
      //create a new thread and increment the counter m.
      let counter = Arc::clone(&m);
      let handle = thread::spawn(move || {
         let mut n = counter.lock().unwrap();
         *n = i;
         println!("counter = {}",*n);
      });
      handles.push(handle);
   }

   for handle in handles {
      handle.join().unwrap();
   }

}
