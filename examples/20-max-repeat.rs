use std::{
    cmp::{self, max},
    collections::BinaryHeap,
    io::{self, *},
};

fn main() {
    let mut container = Vec::with_capacity(10_001);
    let stdin = io::stdin();
    let mut head = true;
    let mut t_num = 0;
    let mut counter = 0;

    for line in stdin.lock().lines() {
        let ll = line.unwrap();

        if head {
            t_num = ll.trim().parse().unwrap();
            head = false;
        } else {
            if counter < t_num {
                let numbers: Vec<&str> = ll.split(" ").collect();
                container.push([
                    numbers[0].trim().parse::<i32>().unwrap_or(0),
                    numbers[1].trim().parse::<i32>().unwrap_or(0),
                ]);
                counter += 1;

                if counter >= t_num {
                    println!("{}", compute(&mut container));
                    container.clear();
                    head = true;
                    counter = 0;
                    t_num = 0;
                }
            }
        }
    }

    stdout().flush().unwrap();
}

fn compute(arr: &mut [[i32; 2]]) -> i32 {
    let mut small_heap: BinaryHeap<cmp::Reverse<i32>> = BinaryHeap::new();
    arr.sort_by(|a, b| a[0].cmp(&b[0]));
    let mut res = 0;
    for sub_arr in arr {
        let (start, end) = (sub_arr[0], sub_arr[1]);
        while let Some(head) = small_heap.peek() {
            if head.0 <= start {
                small_heap.pop();
            } else {
                break;
            }
        }
        small_heap.push(cmp::Reverse(end));
        res = max(res, small_heap.len() as i32);
    }
    res
}
