use indicatif::{ProgressBar, ProgressStyle};
use rand::{thread_rng, Rng};
use std::cmp::min;
use std::time::Duration;

pub fn progress(max_count: u64, interval: Duration, message: String, done_message: String) {
    let mut rng = thread_rng();

    let mut count = 0;

    let pb = ProgressBar::new(max_count);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:.cyan/blue}] {pos}/{len} {wide_msg}",
        )
        .unwrap(),
    );
    pb.set_message(message);

    while count < max_count {
        let diff = max_count - count;
        count += rng.gen_range(1..=min(diff, 10));
        pb.set_position(count);
        std::thread::sleep(interval);
    }
    pb.finish_with_message(done_message);

    std::thread::sleep(interval * 10);
}
