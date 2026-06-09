use std::time::Duration;

use futures::future;

async fn add(a: i32, b: i32) -> i32 {
    println!("Adding {} and {}", a, b);
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    println!("Finished adding {} and {}", a, b);
    a + b
}

fn add2(a: i32, b: i32) -> Number {
    Number {
        a,
        b,
        started: false,
        sleep: Box::pin(tokio::time::sleep(Duration::from_secs(1))),
    }
}

struct Number {
    a: i32,
    b: i32,
    started: bool,
    sleep: std::pin::Pin<Box<tokio::time::Sleep>>,
}

impl Future for Number {
    type Output = i32;

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        println!("Polling to add {} and {}", self.a, self.b);
        if !self.started {
            println!("Starting to add {} and {}", self.a, self.b);
            self.started = true;
        }
        // match std::pin::Pin::new(&mut self.sleep).poll(cx) {
        //    std::task::Poll::Ready(_) => std::task::Poll::Ready(self.a + self.b),
        //    std::task::Poll::Pending => std::task::Poll::Pending,
        // }
        if self.sleep.as_mut().poll(cx).is_pending() {
            println!("Still waiting to add {} and {}", self.a, self.b);
            return std::task::Poll::Pending;
        }
        println!("Finished to add {} and {}", self.a, self.b);
        std::task::Poll::Ready(self.a + self.b)
    }
}

#[tokio::main]
async fn main() {

    // Part 1:
    let result = add(5, 10).await;
    println!("** Part 1 : The result is: {}", result);

    // Part 2:
    let result1 = add2(3, 4).await;
    let result2 = add2(5, 6).await;
    let result3 = add2(7, 8).await;

    println!("** Part 2 : Results: {}, {}, {}", result1, result2, result3);

    // Part 3:
    let mut futures = Vec::new();
    for i in 0..5 {
        futures.push(add2(i, i * 2));
    }

    // Alt code 1 for join_all:
    // let joined_future = future::join_all(futures);
    // joined_future.await;

    // Alt code 2 for join_all:
    let results = future::join_all(futures).await;
    println!("** Part 3 :Results: {:?}", results);
}
