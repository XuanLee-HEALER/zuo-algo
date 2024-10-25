fn main() {
    println!("{}", Solution::longest_subarray(vec![8, 2, 4, 7], 4));
}

struct Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut deq = vec![0; nums.len()];
        let mut h = 0;
        let mut t = 0;

        for i in 0..k - 1 {
            while t > h && nums[deq[t - 1]] <= nums[i as usize] {
                t -= 1;
            }
            deq[t] = i as usize;
            t += 1;
        }

        let mut ans = vec![0; nums.len() - k as usize + 1];

        let mut l = 0;
        let mut r = k as usize - 1;
        while r < nums.len() {
            while t > h && nums[deq[t - 1]] <= nums[r] {
                t -= 1;
            }
            deq[t] = r;
            t += 1;

            ans[l] = nums[deq[h]];

            if deq[h] == l {
                h += 1;
            }
            l += 1;
            r += 1;
        }

        ans
    }

    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let mut max_q = MonotonicQueue::new_max(nums.len());
        let mut min_q = MonotonicQueue::new_min(nums.len());

        let mut ans = 0;
        let mut l = 0;
        let mut r = 0;
        while l < nums.len() {
            while r < nums.len() {
                let cur_max = if let Some(m) = max_q.max() {
                    nums[m].max(nums[r])
                } else {
                    nums[r]
                };
                let cur_min = if let Some(m) = min_q.min() {
                    nums[m].min(nums[r])
                } else {
                    nums[r]
                };
                if cur_max - cur_min <= limit {
                    max_q.push(&nums, r);
                    min_q.push(&nums, r);
                    r += 1;
                } else {
                    break;
                }
            }

            ans = ans.max((r - l) as i32);
            max_q.pop(l);
            min_q.pop(l);
            l += 1;
        }

        ans
    }
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

    fn push(&mut self, ori_arr: &[i32], new_v: usize) {
        match self.typo {
            MonotonicQueueType::Max => {
                while self.t > self.h && ori_arr[self.q[self.t - 1]] <= ori_arr[new_v] {
                    self.t -= 1;
                }
                self.q[self.t] = new_v;
                self.t += 1;
            }
            MonotonicQueueType::Min => {
                while self.t > self.h && ori_arr[self.q[self.t - 1]] >= ori_arr[new_v] {
                    self.t -= 1;
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
