//--------------------------------------
//           Scoped Threads
//--------------------------------------

/*
Scoped Threads allows the threads to borrow a local variable more effectively, as if we use the
`move` keyword, then we cannot make use of that variable in that scope anymore, as move of that
variable has occurred inside of the thread scope.

In particular, it provides clearer control over the lifetime of borrowed variables.
*/

use std::thread;

fn main() {
    let mut vec = vec![1, 2, 3];

    // The below way of using threads causes the `vec` variable to be unable to use later in the code.
    // thread::spawn(|| {
    //     println!("{:?}", vec);
    // });

    /*
    Using the thread::scope function to create scoped threads. The input to the scope closure can be used
    to later spawn threads.

    All the threads that resides inside a thread scope will remain in that scope and will complete its execution
    in that scope.
    */
    thread::scope(|some_scope| {
        some_scope.spawn(|| {
            println!("Thread inside scope.");
            println!("vec: {:?}", vec);
        });

        some_scope.spawn(|| {
            println!("Another thread inside scope");
            // vec.push(4); // Cannot have mutable and immutable references at the same time, as the previous
            // thread may live longer than this one.
            println!("vec: {:?}", vec);
        });
    });

    // These lines will only be executed when the scope ends. At the end of the thread scope, there are no
    // references to `vec` variable, so we can use it in the below code.
    println!("Thread scope finished");
    vec.push(5);
    println!("{:?}", vec);
}
