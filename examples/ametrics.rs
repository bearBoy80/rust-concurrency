use rust_concurrency::AMetrics;
use std::{thread, time::Duration};

fn main() {
    static NANMES: [&str; 5] = ["a", "b", "c", "d", "e"];
    let ametrics = AMetrics::new(&NANMES);
    worker_thread(ametrics.clone(), &NANMES);
    request_worker(ametrics.clone(), &NANMES);
    loop {
        thread::sleep(Duration::from_secs(3));
        println!("{}", ametrics);
    }
}
fn worker_thread(ametrics: AMetrics, names: &'static [&'static str]) {
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(1));
        for (i, item) in names.iter().enumerate() {
            if (i % 2) == 0 {
                let _ = ametrics.inc(item);
            }
        }
    });
}
fn request_worker(ametrics: AMetrics, names: &'static [&'static str]) {
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(2));
        for item in names.iter() {
            let _ = ametrics.inc(item);
        }
    });
}
