//----------------------------------------
//              Barriers
//----------------------------------------

/*
Barriers enable multiple threads to synchronize the beginning of some computation. It is a point in the code
which halts the execution of calling threads until all the threads have executed the code upto that particular
point.

Imagine that the `Task 2` below depends on the `Task 1`. In such an scenario, we will need to make the threads
wait until `Task 1` is completed. For that we use the `wait` function on Barrier.
*/

use std::{
    sync::{Arc, Barrier, Mutex},
    thread,
};

fn main() {
    let mut threads_vec = Vec::new();
    let tasks = Arc::new(Mutex::new(vec![]));
    // The barrier will be used in multiple threads, so it wrapped around by an Arc smart pointer.
    let barrier = Arc::new(Barrier::new(5));

    for i in 0..5 {
        let task = tasks.clone();
        let barrier_clone = barrier.clone();

        let handle = thread::spawn(move || {
            // Task 1
            task.lock()
                .unwrap()
                .push(format!("Thread {i} completed its part on Task 1"));

            barrier_clone.wait(); // This function here, makes the thread wait
                                  // until Task 1 has been completed by 5 threads.

            // Task2
            task.lock()
                .unwrap()
                .push(format!("Thread {i} completed its part on Task 2"));
        });

        threads_vec.push(handle);
    }

    for handle in threads_vec {
        handle.join().unwrap();
    }

    let task_lock = &*tasks.lock().unwrap();
    for task in task_lock {
        println!("{task}");
    }
}
