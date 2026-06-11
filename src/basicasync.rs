async fn doit(n: usize) -> usize {
    println!("*** Doing work for {}", n);
    n * 2
}
#[tokio::main]
async fn main() {
    let mut handles = Vec::new();
    for i in 1..11 {
        let handle = tokio::spawn(async move {
            let result = doit(i).await;
            println!("Task {}: Result: {}", i, result);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}