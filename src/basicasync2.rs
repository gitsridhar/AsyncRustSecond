async fn doit(n: usize) -> usize {
    println!("*** Doing work for {}", n);
    tokio::time::sleep(std::time::Duration::from_secs(1)).await; // All at once, with concurrency
    //std::thread::sleep(std::time::Duration::from_secs(1)); // One at a time, no concurrency
    n * 2
}
#[tokio::main]
async fn main() {
    let mut handles = Vec::new();
    for i in 1..1000000 {
        handles.push(doit(i));
    }
    let joined_futures = futures::future::join_all(handles);
    joined_futures.await;
}