use std::{
    cmp::Ordering,
    io::{self, BufRead, BufReader, BufWriter, Write},
};

struct DropLoc(i32, i32);

impl Ord for DropLoc {
    fn cmp(&self, other: &Self) -> Ordering {
        self.1.cmp(&other.1)
    }
}

impl PartialOrd for DropLoc {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for DropLoc {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl Eq for DropLoc {}

fn main() {
    let mut br = BufReader::new(io::stdin());
    let mut bw = BufWriter::new(io::stdout());
    let mut buf = String::new();
    let mut head = false;
    let mut n = 0;
    let mut d = 0;
    let mut drop: Vec<DropLoc> = Vec::new();
    while let Ok(sz) = br.read_line(&mut buf) {
        if sz > 0 {
            if !head {
                let mut segs = buf.trim().split(" ");
                n = segs.next().unwrap().parse().unwrap();
                d = segs.next().unwrap().parse().unwrap();
                head = true;
            } else {
                let mut segs = buf.trim().split(" ");
                drop.push(DropLoc(
                    segs.next().unwrap().parse().unwrap(),
                    segs.next().unwrap().parse().unwrap(),
                ));
                n -= 1;

                if n == 0 {
                    drop.sort_by(|a, b| a.0.cmp(&b.0));
                    let res = compute(&drop, d);
                    bw.write_fmt(format_args!("{}\n", if res == i32::MAX { -1 } else { res }))
                        .unwrap();
                    drop.clear();
                    head = false;
                }
            }
        } else {
            break;
        }
        buf.clear();
    }
    bw.flush().unwrap();
}

fn compute(drop: &[DropLoc], limit: i32) -> i32 {
    let mut ans = i32::MAX;
    let mut max_q = MonotonicQueue::new_max(drop.len());
    let mut min_q = MonotonicQueue::new_min(drop.len());
    let n = drop.len();

    // 只需要判断花盆范围内是否满足即可，不满足就进
    let ok = |max_q: &MonotonicQueue, min_q: &MonotonicQueue| -> bool {
        let cur_max = if let Some(max) = max_q.max() {
            drop[max].1
        } else {
            0
        };
        let cur_min = if let Some(min) = min_q.min() {
            drop[min].1
        } else {
            0
        };
        cur_max - cur_min >= limit
    };
    let mut l = 0;
    let mut r = 0;
    while l < drop.len() {
        while r < drop.len() {
            if !ok(&max_q, &min_q) {
                max_q.push(drop, r);
                min_q.push(drop, r);
                r += 1;
            } else {
                break;
            }
        }

        if ok(&max_q, &min_q) {
            ans = ans.min(drop[r - 1].0 - drop[l].0);
        }

        max_q.pop(l);
        min_q.pop(l);
        l += 1;
    }

    ans
}

#[derive(PartialEq, Eq)]
enum MonotonicQueueType {
    Min,
    Max,
}

struct MonotonicQueue {
    h: usize,
    t: usize,
    q: Vec<usize>,
    typo: MonotonicQueueType,
}

impl MonotonicQueue {
    fn new_max(len: usize) -> Self {
        Self {
            h: 0,
            t: 0,
            q: vec![0; len],
            typo: MonotonicQueueType::Max,
        }
    }

    fn new_min(len: usize) -> Self {
        Self {
            h: 0,
            t: 0,
            q: vec![0; len],
            typo: MonotonicQueueType::Min,
        }
    }

    fn max(&self) -> Option<usize> {
        if self.t > self.h && self.typo == MonotonicQueueType::Max {
            Some(self.q[self.h])
        } else {
            None
        }
    }

    fn min(&self) -> Option<usize> {
        if self.t > self.h && self.typo == MonotonicQueueType::Min {
            Some(self.q[self.h])
        } else {
            None
        }
    }

    fn push<T: Ord>(&mut self, ori_arr: &[T], new_v: usize) {
        match self.typo {
            MonotonicQueueType::Max => {
                while self.t > self.h {
                    match ori_arr[self.q[self.t - 1]].cmp(&ori_arr[new_v]) {
                        Ordering::Less | Ordering::Equal => self.t -= 1,
                        _ => break,
                    }
                }
                self.q[self.t] = new_v;
                self.t += 1;
            }
            MonotonicQueueType::Min => {
                while self.t > self.h {
                    match ori_arr[self.q[self.t - 1]].cmp(&ori_arr[new_v]) {
                        Ordering::Greater | Ordering::Equal => self.t -= 1,
                        _ => break,
                    }
                }
                self.q[self.t] = new_v;
                self.t += 1;
            }
        }
    }

    fn pop(&mut self, old_v: usize) {
        if self.t > self.h && old_v == self.q[self.h] {
            self.h += 1;
        }
    }
}

#[cfg(test)]
mod monotonic_queue_test {

    #[test]
    fn test_max_monotonic_queue() {
        let arr = vec![3, 5, -2, 7, 1, 4, 2, 9, 2, 1];
        let mut max_q = super::MonotonicQueue::new_max(10);
        assert_eq!(None, max_q.max());
        max_q.push(&arr, 0);
        assert_eq!(Some(0), max_q.max());
        max_q.push(&arr, 1);
        assert_eq!(Some(1), max_q.max());
        max_q.push(&arr, 2);
        assert_eq!(Some(1), max_q.max());
        max_q.push(&arr, 3);
        assert_eq!(Some(3), max_q.max());
        assert_eq!(None, max_q.min());
        max_q.pop(0);
        assert_eq!(Some(3), max_q.max());
        max_q.pop(1);
        assert_eq!(Some(3), max_q.max());
        max_q.pop(2);
        assert_eq!(Some(3), max_q.max());
    }

    fn test_min_monotonic_queue() {
        let arr = vec![3, 5, -2, 7, 1, 4, 2, 9, 2, 1];
        let mut min_q = super::MonotonicQueue::new_min(10);
        assert_eq!(None, min_q.min());
        min_q.push(&arr, 0);
        assert_eq!(Some(0), min_q.min());
        min_q.push(&arr, 1);
        assert_eq!(Some(0), min_q.min());
        min_q.push(&arr, 2);
        assert_eq!(Some(2), min_q.min());
        min_q.push(&arr, 3);
        assert_eq!(Some(2), min_q.min());
        assert_eq!(None, min_q.max());
        min_q.pop(0);
        assert_eq!(Some(2), min_q.min());
        min_q.pop(1);
        assert_eq!(Some(2), min_q.min());
        min_q.pop(2);
        assert_eq!(Some(3), min_q.min());
    }
}
