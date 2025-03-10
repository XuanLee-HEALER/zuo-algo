use std::collections::BinaryHeap;

fn main() {
    println!("{}", Solution::largest_palindromic("6006".into()))
}

struct Solution;

impl Solution {
    pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
        let n = dist.len();
        let mut time = Vec::with_capacity(n);
        let mut i = 0;
        while i < n {
            time.push((dist[i] + speed[i] - 1) / speed[i]);
            i += 1;
        }
        time.sort_unstable();
        i = 0;
        while i < n {
            if time[i] <= i as i32 {
                return i as i32;
            }
            i += 1;
        }
        n as i32
    }

    pub fn largest_palindromic(num: String) -> String {
        let num = num.as_bytes();
        let mut count = vec![0; b'9' as usize + 1];
        num.iter().for_each(|&b| count[b as usize] += 1);
        let mut res = vec![0_u8; num.len()];
        let mut left = 0;
        let mut mid = 0;
        for b in (b'1'..=b'9').rev() {
            let b = b as usize;
            if mid == 0 && count[b] & 1 == 1 {
                mid = b as u8;
            }
            for _ in 0..(count[b] / 2) {
                res[left] = b as u8;
                left += 1;
            }
        }
        if left == 0 {
            if mid == 0 {
                return String::from("0");
            } else {
                return format!("{}", char::from_u32(mid as u32).unwrap());
            }
        } else {
            if mid == 0 && count[b'0' as usize] & 1 == 1 {
                mid = b'0'
            }
            for _ in 0..(count[b'0' as usize] / 2) {
                res[left] = b'0';
                left += 1;
            }
        }
        let end = left - 1;
        if mid != 0 {
            res[left] = mid;
            left += 1;
        }
        for i in (0..=end).rev() {
            res[left] = res[i];
            left += 1;
        }
        String::from_utf8(res[..left].to_vec()).unwrap()
    }

    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        struct Class(f64, f64, f64);
        impl Ord for Class {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.2.partial_cmp(&other.2).unwrap()
            }
        }
        impl PartialOrd for Class {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Eq for Class {}
        impl PartialEq for Class {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0 && self.1 == other.1 && self.2 == other.2
            }
        }
        let mut max_heap = BinaryHeap::new();
        let n = classes.len();
        for class in classes {
            let class0 = class[0] as f64;
            let class1 = class[1] as f64;
            max_heap.push(Class(
                class0,
                class1,
                ((class0 + 1.0) / (class1 + 1.0)) - (class0 / class1),
            ));
        }
        for _ in 0..extra_students {
            let mut top = max_heap.pop().unwrap();
            top.0 += 1.0;
            top.1 += 1.0;
            top.2 = ((top.0 + 1.0) / (top.1 + 1.0)) - (top.0 / top.1);
            max_heap.push(top);
        }
        let mut res = 0_f64;
        while let Some(c) = max_heap.pop() {
            res += c.0 / c.1
        }
        res / n as f64
    }

    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        struct Worker(f64, i32);
        impl Eq for Worker {}
        impl PartialEq for Worker {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0 && self.1 == other.1
            }
        }
        impl Ord for Worker {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.1.cmp(&other.1)
            }
        }
        impl PartialOrd for Worker {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        let mut workers = quality
            .iter()
            .zip(wage.iter())
            .map(|(&q, &w)| Worker(w as f64 / q as f64, q))
            .collect::<Vec<Worker>>();
        workers.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        let mut max_heap = BinaryHeap::new();
        let mut res = f64::MAX;
        let mut k_sum = 0_f64;
        let mut num = 0;
        for worker in workers {
            if num < k {
                k_sum += worker.1 as f64;
                num += 1;
                if num == k {
                    res = res.min(worker.0 * k_sum)
                }
                max_heap.push(worker);
            } else {
                if worker.1 < max_heap.peek().unwrap().1 {
                    let old = max_heap.pop().unwrap();
                    k_sum += (worker.1 - old.1) as f64;
                    res = res.min(worker.0 * k_sum);
                    max_heap.push(worker);
                }
                num += 1;
            }
        }
        res
    }
}
