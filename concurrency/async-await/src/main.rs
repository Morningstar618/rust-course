//--------------------------------------------
//               Async Await
//--------------------------------------------

/*
Typical code in rust is synchronous i.e. it executes line by line, without yielding control back
to the runtime. Rust also provides us the ability to write asynchronous code, which allows us to
write functions, blocks and closures that can pause execution and yield control back to the runtime,
allowing other code to make progress and then pick back from where it left off.

The pauses to wait for are generally for some input or output to occur. Due to yeilding nature, it is
also sometimes known as cooperative scheduling, as it gives the control back to the executor or runtime,
thereby cooperatively allowing others to make progress and execute.

The `async` keyword will make a function asynchronous. This changes the behaviour of the code, in terms
of how it will execute. This function returns something that implements the future trait with an associated
type of output representing the function output.

A Future trait has many details, but at an abstract level it has a poll method, which can either return a
ready state when a return value is available or pending, indicating that the value is currently not available.
The executor/runtime will poll it after some interval to check again the status of the future. The executor
will stop polling a future once a value becomes available.  The futures are like promises, which is giving
you the promise that the function will generate some value in future. We don't know when, but all we know
is that it will provide some value in the future.

NOTE: The futures can only start to execute and work on to generate the value when we `await` on it. It
derives the future to completion. This is because in rust futures are `lazy`, meaning they won't do
anything unless driven to completion.

`await` is only allowed inside async functions and blocks.

The standard library does not provide an async runtime, but community does, i.e. `tokio`. Tokio is a runtime
that allows main function to go to completion. We have to use tokio as using await on the main function instead
returns another error.

There are some benefits for the futures to be lazy. First is that they are a zero cost abstraction. This
means you won't incur a runtime cost unless you actually use them. Secondly, they are easy to cancel. To do
so we can call the drop method on the future. Do so, and the future will not be polled in the remaining code.
*/

async fn printing() {
    println!("This is an asynchronous function");
}

#[tokio::main]
async fn main() {
    let x = printing().await;
    println!("The future has not been polled yet");
    // drop(x);

    // x.await;
}
