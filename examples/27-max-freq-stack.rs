use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let stk = FreqStack::new();
}

struct FreqStack {
    max_freq: usize,
    mp: HashMap<i32, usize>,
    vc: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FreqStack {
    fn new() -> Self {
        Self {
            max_freq: 0,
            mp: HashMap::new(),
            vc: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        match self.mp.entry(val) {
            std::collections::hash_map::Entry::Occupied(ref mut e) => {
                let cur_max_freq = self.vc.len();
                let new_freq = *e.get() + 1;
                match cur_max_freq.cmp(&new_freq) {
                    Ordering::Less => {
                        self.vc.push(vec![val]);
                        self.max_freq = new_freq;
                    }
                    _ => {
                        self.vc[new_freq - 1].push(val);
                    }
                }
                *e.get_mut() += 1;
            }
            std::collections::hash_map::Entry::Vacant(_) => {
                if self.vc.is_empty() {
                    self.max_freq = 1;
                    self.vc.push(vec![val])
                } else {
                    self.vc[0].push(val)
                }
                self.mp.insert(val, 1);
            }
        }
    }

    fn pop(&mut self) -> i32 {
        let max_freq_idx = self.max_freq - 1;
        let v = self.vc[max_freq_idx].pop().unwrap();
        if self.vc[max_freq_idx].is_empty() {
            self.vc.pop();
            self.max_freq -= 1;
        }
        *(self.mp.get_mut(&v).unwrap()) -= 1;
        v
    }
}
