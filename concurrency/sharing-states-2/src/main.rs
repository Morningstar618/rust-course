//--------------------------------------
//      Sharing States (Part 2)
//--------------------------------------

use std::{
    sync::{Arc, Mutex},
    thread,
};

#[derive(Debug)]
struct File {
    text: Vec<String>,
}

/*
In the below code, we are making use of a smart pointer because we need multiple threads to own the
same copy of the `File` type and add text in that. We cannot use clone because that will create separate
copies of `File` type in the heap for each thread, thus the requirement of using a Smart Pointer.

Additionally, Arc smart pointer is being used and not Rc because the latter is not thread safe, meaning
it cannot be sent between threads in a safe way. Arc was used in order to get thread safe shared ownership.

Arc stands for the Atomically Reference Counted.

Rc smart pointer manages a reference count for keeping track of the owners. It adds to the count for each call
to clone, and subtracts from the count when a clone or an owner is dropped, but it doesn't use any concurrency
primitives to make sure that changes to the count can't be interrupted by another thread, and therefore will
always have a correct value.

Whereas, the Arc smart pointer makes sure that the reference count are updated in a consistent manner between
the threads.

NOTE: The variable `file` is not mutable, however inside the loop we are able to mutate its value. This is because
`Mutex` uses interior mutability just like the `RefCell` smart pointer.
*/

fn main() {
    let file = Arc::new(Mutex::new(File { text: vec![] }));
    let mut thread_vec = vec![];

    for i in 0..10 {
        let file = Arc::clone(&file);
        let handle = thread::spawn(move || {
            let mut file_lock = file.lock().unwrap();
            file_lock.text.push(format!("Hello from Thread {}", i));
        });

        thread_vec.push(handle);
    }

    for handle in thread_vec {
        handle.join().unwrap();
    }

    let file_lock = file.lock().unwrap();

    for t in &file_lock.text {
        println!("{t}");
    }
}
