use std::time::Instant;

use rand::{seq::SliceRandom, thread_rng};

fn main() {
    let mut rng = thread_rng();
    let mut seq: Vec<i32> = (1..100).collect();
    seq.shuffle(&mut rng);

    println!("seq before sort:\n{:?}", seq);

    let cur_time = Instant::now();
    seq.sort_by(|a, b| b.cmp(a));
    println!(
        "seq after sort:\n{:?}\n{}s",
        seq,
        cur_time.elapsed().as_secs()
    )
}
