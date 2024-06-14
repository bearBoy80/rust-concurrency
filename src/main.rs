use futures::executor::block_on;

fn main() {
    let future = do_something(); // Nothing is printed
    block_on(future);
}
async fn do_something() {
    println!("hello, world!");
}
