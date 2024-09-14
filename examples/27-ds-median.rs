use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    let mut f = MedianFinder::new();

    f.add_num(1);
    f.add_num(2);
    println!("{}", f.find_median());
    f.add_num(3);
    println!("{}", f.find_median());
    f.add_num(4);
    println!("{}", f.find_median());
}

struct MedianFinder {
    count: usize,
    median: f64,
    big_heap: BinaryHeap<i32>,
    small_heap: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    fn new() -> Self {
        Self {
            count: 0,
            median: 0_f64,
            big_heap: BinaryHeap::new(),
            small_heap: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        match (self.big_heap.len(), self.big_heap.len()) {
            (0, 0) => {
                self.big_heap.push(num);
                self.count += 1;
                self.median = num as f64;
            }
            (_, _) => {
                if num > *self.big_heap.peek().unwrap() {
                    self.small_heap.push(Reverse(num));
                } else {
                    self.big_heap.push(num);
                }
                self.count += 1;
                self.balance()
            }
        }
    }

    fn balance(&mut self) {
        let (big, small) = (self.big_heap.len(), self.small_heap.len());
        if big > small {
            if big - small >= 2 {
                self.small_heap.push(Reverse(self.big_heap.pop().unwrap()))
            }
        } else if small - big >= 2 {
            self.big_heap.push(self.small_heap.pop().unwrap().0)
        }

        if self.count % 2 == 0 {
            self.median =
                (self.big_heap.peek().unwrap() + self.small_heap.peek().unwrap().0) as f64 / 2_f64;
        } else {
            self.median = if big > small {
                *(self.big_heap.peek().unwrap()) as f64
            } else {
                self.small_heap.peek().unwrap().0 as f64
            }
        }
    }

    fn find_median(&self) -> f64 {
        self.median
    }
}
