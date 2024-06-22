//---------------------------------------------------
//     Message Passing through Channels (Part 2)
//---------------------------------------------------
use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let x = "Hello".to_string();
        println!("Sending value!");
        thread::sleep(Duration::from_secs(3));
        tx.send(x).unwrap();
    });

    /* This receive implementation will block the main thread
        let recv_val = rx.recv();
        // The below code will not be executed for 3 seconds as the main thread is blocked for
        // 3 seconds by the above line of code when receiving the value.
        println!("I am Blocked.");
    */

    // This receive implementation will not block the main thread
    let mut received_status = false;
    while received_status != true {
        // The try_recv method will return the value (if present) when called or will return an
        // error (if value is absent)
        match rx.try_recv() {
            Ok(received_value) => {
                println!("Received value: {}", received_value);
                received_status = true;
            }

            #[allow(unused_variables)]
            Err(err) => {
                println!("I am doing something.");
                // Other code that we want running while the receiver is listening can be wrote here
            }
        }
    }

    println!("***********Did this work?**************");
}
