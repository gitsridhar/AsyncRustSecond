use futures::executor::block_on;
use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Wake, Waker},
    thread,
    time::Duration,
};

fn main() {
    println!("--- block_on + timer-thread wakeup ---");
    block_on(sleep(Duration::from_secs(5)));
    println!("5 seconds have passed!");

    println!("\n--- mini executor poll loop + timer-thread wakeup ---");
    mini_block_on(sleep(Duration::from_secs(2)));
    println!("2 seconds have passed (mini executor)!");
}

fn sleep(duration: Duration) -> Sleep {
    Sleep::new(duration)
}

struct Sleep {
    shared_state: Arc<Mutex<SharedState>>,
}

struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}

impl Sleep {
    fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        let thread_shared_state = Arc::clone(&shared_state);
        thread::spawn(move || {
            thread::sleep(duration);

            let mut state = thread_shared_state
                .lock()
                .expect("timer shared state poisoned");
            state.completed = true;
            if let Some(waker) = state.waker.take() {
                waker.wake();
            }
        });

        Self { shared_state }
    }
}

impl Future for Sleep {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self
            .shared_state
            .lock()
            .expect("timer shared state poisoned");

        if state.completed {
            println!("Wake time reached!");
            Poll::Ready(())
        } else {
            println!("Not yet time to wake up, sleeping...");
            // Store the most recent task waker so the timer thread can wake it.
            state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

struct ThreadWaker {
    thread: thread::Thread,
}

impl Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        self.thread.unpark();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.thread.unpark();
    }
}

fn mini_block_on<F: Future>(future: F) -> F::Output {
    let current_thread = thread::current();
    let thread_waker = Arc::new(ThreadWaker {
        thread: current_thread,
    });
    let waker: Waker = Waker::from(thread_waker);
    let mut context = Context::from_waker(&waker);

    let mut future = Box::pin(future);

    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(output) => return output,
            Poll::Pending => {
                println!("mini executor: pending; parking thread until wakeup...");
                thread::park();
                println!("mini executor: received wakeup; polling again...");
            }
        }
    }
}