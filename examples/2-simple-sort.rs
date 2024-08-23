use std::time::Instant;

use rand::{seq::SliceRandom, thread_rng};

fn selection_sort(arr: &mut [i32]) {
    if arr.len() >= 2 {
        let mut i = 0;
        while i < arr.len() {
            let mut min = i;
            let mut j = i;
            while j < arr.len() {
                if arr.get(j).unwrap() < arr.get(min).unwrap() {
                    min = j;
                }
                j += 1;
            }
            arr.swap(i, min);
            i += 1;
        }
    }
}

fn bubble_sort(arr: &mut [i32]) {
    if arr.len() >= 2 {
        let mut end = arr.len() - 1;
        while end > 0 {
            let mut i = 0;
            let mut j = 1;
            while j <= end {
                let prev = arr.get(i).unwrap();
                let next = arr.get(j).unwrap();
                if prev > next {
                    arr.swap(i, j);
                }
                i += 1;
                j += 1;
            }
            end -= 1;
        }
    }
}

fn insert_sort(arr: &mut [i32]) {
    if arr.len() >= 2 {
        let mut foreach_count = 0;

        for i in 1..arr.len() {
            foreach_count += 1;
            for j in (0..=i - 1).rev() {
                foreach_count += 1;
                let next_val = arr.get(j + 1).unwrap();
                let prev_val = arr.get(j).unwrap();
                if next_val < prev_val {
                    arr.swap(j, j + 1);
                }
            }
        }

        println!("for count => {}", foreach_count)
    }
}

fn insert_sort_1(arr: &mut [i32]) {
    if arr.len() >= 2 {
        let mut foreach_count = 0;

        let mut i = 0;
        while i < arr.len() - 1 {
            foreach_count += 1;
            let mut j = i + 1;
            while j > 0 {
                foreach_count += 1;
                let next_val = arr.get(j).unwrap();
                let prev_val = arr.get(j - 1).unwrap();
                if next_val < prev_val {
                    arr.swap(j, j - 1);
                } else {
                    break;
                }
                j -= 1;
            }
            i += 1;
        }
        println!("for count => {}", foreach_count)
    }
}

fn count_time_sort<F>(sort: F, arr: &mut [i32])
where
    F: FnOnce(&mut [i32]),
{
    let start = Instant::now();
    // println!("seq before sort:\n{:?}", arr);
    sort(arr);
    // println!("seq after sort:\n{:?}", arr);
    println!("elapsed:\n{}s", start.elapsed().as_secs());
}

fn main() {
    let mut rng = thread_rng();
    let mut seq: Vec<i32> = (1..20000).collect();
    seq.shuffle(&mut rng);
    println!("shuffle finished");
    // println!("seq before sort:\n{:?}", seq);

    // default sort
    // seq.sort_by(|a, b| b.cmp(a));
    // selection sort
    // selection_sort(&mut seq);
    // bubble sort
    // count_time_sort(bubble_sort, &mut seq);
    // select sort
    count_time_sort(insert_sort, &mut seq.clone());
    count_time_sort(insert_sort_1, &mut seq.clone());
}
