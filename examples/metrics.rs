use std::{thread, time::Duration};

use rand::Rng;
use rust_concurrency::Metrics;

fn main() {
    let metrics = Metrics::new();
    let mut rng = rand::thread_rng();

    for i in 0..10 {
        worker(i, metrics.clone());
    }
    for _ in 0..100 {
        let idx = rng.gen_range(1..=10);
        request(idx, metrics.clone());
    }
    loop {
        thread::sleep(Duration::from_secs(2));
        println!("{}", metrics);
    }
}
fn worker(idx: usize, metrics: Metrics) {
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(1));
        let _ = metrics.inc(format!("page.index.idx.{}", idx));
    });
}
fn request(idx: usize, metrics: Metrics) {
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(1));
        let _ = metrics.inc(format!("page.index.idx.{}", idx));
    });
}
