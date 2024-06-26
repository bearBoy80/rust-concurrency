#![allow(unused)]
fn main() {
    use core::pin::pin;
    use std::future::Future;
    use std::sync::Arc;
    use std::task::{Context, Poll, Wake};
    use std::thread::{self, Thread};

    /// A waker that wakes up the current thread when called.
    struct ThreadWaker(Thread);

    impl Wake for ThreadWaker {
        fn wake(self: Arc<Self>) {
            self.0.unpark();
        }
    }

    /// Run a future to completion on the current thread.
    fn block_on<T>(fut: impl Future<Output = T>) -> T {
        // Pin the future so it can be polled.
        let mut fut = pin!(fut);

        // Create a new context to be passed to the future.
        let t = thread::current();
        let waker = Arc::new(ThreadWaker(t)).into();
        let mut cx = Context::from_waker(&waker);

        // Run the future to completion.
        loop {
            match fut.as_mut().poll(&mut cx) {
                Poll::Ready(res) => {
                    println!("----");
                    return res;
                }
                Poll::Pending => thread::park(),
            }
        }
    }

    block_on(async {
        println!("Hi from inside a future!");
    });
}
