use std::{future::Future, sync::Arc, thread, time::Duration};

use futures::{
    executor::block_on,
    lock::{Mutex, MutexGuard},
};
use tokio::task::yield_now;
#[tokio::main]
async fn main() {
    let name = "hello world";
    let mute_id = Mutex::new(4);

    sayhello(name).await;
    sayhello1(name).await;
    block_on(sayhello1(name));
    tokio::spawn(async move {
        let rc = Arc::new("hello");
        // `rc` is used after `.await`. It must be persisted to
        // the task's state.
        yield_now().await;
        increment_and_do_stuff(&mute_id).await;
        println!("{}", rc);
    });
    thread::sleep(Duration::from_secs(1));
}
async fn sayhello(name: &str) {
    println!("sayhello : {}", name);
}
#[allow(clippy::manual_async_fn)]
fn sayhello1(name: &str) -> impl Future<Output = ()> + '_ {
    async move {
        println!("sayhello1 : {}", name);
    }
}
async fn increment_and_do_stuff(mutex: &Mutex<i32>) {
    let mut lock: MutexGuard<i32> = mutex.lock().await;
    *lock += 1;

    sayhello(lock.to_string().as_str()).await;
}
