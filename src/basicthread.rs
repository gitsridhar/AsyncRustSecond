fn doit(n: usize) -> usize {
    println!("Doing work for {}", n);
    n * 2
}

fn main() {
    let mut thread_handles = Vec::new();
    for i in 1..11 {
        let handle = std::thread::spawn(move || {
            let result = doit(i);
            println!("Thread {}: Result: {}", i, result);
        });
        thread_handles.push(handle);
    }

    for handle in thread_handles {
        handle.join().unwrap();
    }
}
