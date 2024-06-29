//--------------------------------------------
//              Thread Parking
//--------------------------------------------

/*
Here we are utilizing `Thread Parking` such that `thread 1` prints the data after the same has been
updated by `thread 2`.
*/

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

fn main() {
    let data = Arc::new(Mutex::new(5));
    let data_clone = data.clone();

    let thread_1 = thread::spawn(move || {
        println!("Thread 1: I am doing some work.");
        println!("Thread 1: I am doing some more work!");

        // thread::sleep(Duration::from_secs(2));  // Puts the thread unconditionally into sleep mode.

        println!("Thread 1: Parked");

        // thread::park();  // Put thread in sleep mode until `unpark` is called on it.
        thread::park_timeout(Duration::from_secs(4)); // Either the thread is first `unparked` or later timed out,
                                                      // whichever comes first.

        println!("Thread 1: Printing the updated data");
        println!("Thread 1: Data: {:?}", *data.lock().unwrap());
    });

    let thread_2 = thread::spawn(move || {
        println!("Thread 2: I am working on updating the data");
        thread::sleep(Duration::from_secs(1));

        *data_clone.lock().unwrap() = 10;
        println!("Thread 2: Data updation completed.");
    });

    thread_2.join().unwrap();
    /*
    The line below is commented out to test out the `park_timeout` function where it conditionally
    puts the thread to sleep until the unpark function is called or the timeout duration expires.

    Right now, the thread will be in sleep state till 4 seconds.
    */
    // thread_1.thread().unpark();
    thread_1.join().unwrap();
}
