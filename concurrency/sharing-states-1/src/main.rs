//---------------------------------------
//       Sharing States (Part 1)
//---------------------------------------

/*
State sharing is another approach for sharing data between threads.

Message passing is one-way data flow where a sender thread passes a message to the receiver thread.
The ownership is transfered from the sending thread to the receiving thread.

But with share state concurrency, we have some piece of code residing inside the memory that multiple
threads can access by acquiring lock on the data.This is possible because of a special type called `Mutex`.

`Mutex` stands for mutual exclusion. The data wrapped by a mutex can only be accessed by a single thread at
any given time. To gain access to the data, a locking mechanism is used. When a thread wants to gain access
to a data behind a Mutex, it will acquire a lock on the mutex. Once a lock is aquired no other thread can
access that data. The lock will only be released once the thread is done with the data, allowing other threads
to acquire the lock.
*/

use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        /*
        // The `lock` method will block the current thread until it is able to acquire the lock. If there
        // are multiple threads trying to call the lock, then only the first thread that makes a call will
        // be given access and the remaining threads will be blocked until the lock is released.

        // A call to lock will return a result. If there is already a thread that has acquired a lock, and if
        // that thread panics, then the call to lock will return an error.
        */

        let mut num = m.lock().unwrap();
        *num = 10;

        /*
        Each time you acquire a lock, you have to make sure that you unlock it explicitly in order to make it
        available for other parts of your code. Although the rust type system and ownership rules gaurantees
        that you can't get locking and unlocking wrong.

        The lock will be automatically released once the variable num is dropped or goes out of scope. For
        example, after the code block, the lock can be acquired again.
        */
    }

    let lock_m = m.lock().unwrap();
    println!("m is {:?}", *lock_m);

    drop(lock_m); // Without dropping `lock_m`, the current calling (main) thread will be blocked as further another
                  // lock is being acquired on it below.

    let lock_m1 = m.lock().unwrap();
    println!("This code is blocked");
}
