use futures::future;

async fn add(a: i32, b: i32) -> i32 {
    println!("Adding {} and {}", a, b);
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    println!("Finished adding {} and {}", a, b);
    a + b
}

#[tokio::main]
async fn main() {
    let result = add(5, 10).await;
    println!("The result is: {}", result);

    let mut futures = Vec::new();
    for i in 0..5 {
        futures.push(add(i, i * 2));
    }

    // let joined_future = future::join_all(futures);
    // joined_future.await;

    let results = future::join_all(futures).await;
    println!("Results: {:?}", results);
}
