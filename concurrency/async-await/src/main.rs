//--------------------------------------------
//               Async Await
//--------------------------------------------

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
