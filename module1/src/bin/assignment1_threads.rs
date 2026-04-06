use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..5 {  
        let cnt = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            for _ in 0..10 {  
                let mut num = cnt.lock().unwrap();
                println!("Thread {} at step {}", i, *num);
                *num += 1;
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter: {}", *counter.lock().unwrap());
}