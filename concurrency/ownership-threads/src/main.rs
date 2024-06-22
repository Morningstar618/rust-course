//---------------------------------------------
//          Ownership and Threads
//---------------------------------------------

use std::thread;

/*
Threads in rust are isolated from each other automatically due to ownership. This ensures
that data races will never occur.
*/

fn main() {
    let x = "Hello".to_string();

    thread::spawn(move || {
        println!("{}", x);
    });
}
