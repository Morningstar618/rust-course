use std::{thread, time::Duration};

/*
------------ Concurrency vs Parallelism --------------

Concurrency is about multiple tasks which start, run, and complete in overlapping time periods
in no specific order. Example, multitasking in a single core machine. Concurrency means executing
multiple tasks at the same time but not necessarily simultaneously.

Parallelism is about multiple tasks that run literally at the same time on a hardware with multiple
computing resources like multicore processors.

Parallelism requires hardware with multiple processing units, essentially. In single-core CPU, you may
get concurrency but NOT parallelism.

From programming perspective, we are interested in concurrency and from hardware perspective we are
interested in parallelism.

Concurrency - Doing lot of things at once.
Parallelism - Dealing with lot of things at once.

*/

fn main() {
    println!("This line will be printed");
    println!("This line will also be printed");
    println!("The concurrency will start here after this line");

    let t = thread::spawn(|| {
        println!("Hello 1 from thread");
        println!("Hello 2 from thread");
        println!("Hello 3 from thread");
        println!("Hello 4 from thread");
        println!("Hello 5 from thread");
        println!("Hello 6 from thread");
        println!("Hello 7 from thread");
        println!("Hello 8 from thread");
    });

    // thread::sleep(Duration::from_millis(1));

    println!("Hello 1 from main");
    println!("Hello 2 from main");

    t.join();
}
