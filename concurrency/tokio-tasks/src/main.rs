//-------------------------------------
//           Tokio Tasks
//-------------------------------------

/*
To make our async code run concurrently, we use `tokio tasks`. A task is a light-weight, non-blocking
unit of execution. Tasks allow, top-level futures to execute concurrently.

`spawn` function in the tokio module can be used to spawn new tasks. It expects a `future` as an arguement
and returns a `JoinHandle`.

NOTE: The API for tasks is very similar to the API for spawning a thread. This is on purpose and the intention
is to make it easy to switch from using threads to using tasks.

In Tasks, like threads, the execution is not deterministic, meaning that if you execute again, you may get
different results.

By default, tokio uses the thread pool to execute tasks on multiple threads. We could however force tokio to
run all the tasks on a single thread by changing the flavour to `current_thread`. By doing so, the code will
be executed on a single thread `sequentially`. This is so as the code inside the tasks (the futures inside the handle variable)
always make progress and are immediately resolved, thereby completing the tasks in sequential order. But if
for some reason, the call to printing waits for some IO, then the other tasks will be given a chance as they
are non-blocking. This was tested by calling the `sleep` function inside the `printing` function thereby
giving other tasks a chance to proceed.
*/

use std::time::Duration;

use tokio::time::sleep;

async fn printing(i: i32) {
    sleep(Duration::from_secs(1)).await;
    println!("Task {i}");
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut handles = vec![];
    for i in 0..3 {
        let handle = tokio::spawn(async move {
            println!("Task {i}, printing 1st time");
            printing(i).await;
            println!("Task {i}, printing 2nd time");
            printing(i).await;
            println!("Task {i}, completed");
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    println!("All tasks are  now completed");
}
