use std::collections::{self, BinaryHeap};

fn main() {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.val.partial_cmp(&self.val)
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.val.cmp(&self.val)
    }
}

struct Solution;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut small_heap = collections::BinaryHeap::new();
        for head in lists {
            if let Some(head) = head {
                small_heap.push(*head);
            }
        }
        if small_heap.is_empty() {
            return None;
        }
        let mut res = small_heap.pop().unwrap();
        let mut p = &mut res;

        while !small_heap.is_empty() {
            if let Some(nxt) = p.next.take() {
                small_heap.push(*nxt);
            }

            let t_head = small_heap.pop().unwrap();
            p.next = Some(Box::new(t_head));
            p = p.next.as_mut().unwrap();
        }

        Some(Box::new(res))
    }

    pub fn halve_array(nums: Vec<i32>) -> i32 {
        let mut max_heap = BinaryHeap::new();
        let mut sum: i64 = 0;
        for num in nums {
            max_heap.push((num as i64) << 20);
            sum += num as i64;
        }
        sum <<= 20;

        let goal = sum / 2;
        let mut sub = 0;
        let mut count = 0;
        while !max_heap.is_empty() {
            count += 1;
            let head = max_heap.pop().unwrap();
            let half = head / 2;
            sub += half;
            if sub >= goal {
                break;
            }
            max_heap.push(half)
        }

        count
    }
}
