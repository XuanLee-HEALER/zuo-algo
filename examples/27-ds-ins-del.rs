use std::collections::{HashMap, HashSet};

use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};

fn main() {
    let mut rc = RandomizedCollection::new();
    rc.insert(10);
    rc.insert(10);
    rc.insert(20);
    rc.insert(20);
    rc.insert(30);
    rc.insert(30);
    rc.remove(10);
    rc.remove(10);
    rc.remove(30);
    rc.remove(30);
    println!("{}", rc.get_random());
}

struct RandomizedCollection {
    mp: HashMap<i32, HashSet<usize>>,
    vc: Vec<i32>,
    rng: ThreadRng,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedCollection {
    fn new() -> Self {
        Self {
            mp: HashMap::new(),
            vc: Vec::new(),
            rng: thread_rng(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        self.vc.push(val);

        if let std::collections::hash_map::Entry::Vacant(e) = self.mp.entry(val) {
            e.insert(HashSet::from([self.vc.len() - 1]));
            true
        } else {
            self.mp.get_mut(&val).unwrap().insert(self.vc.len() - 1);
            false
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        if self.mp.contains_key(&val) {
            let l = self.vc.len();
            let tv = self.mp.get(&val).unwrap();
            let t_idx = *tv.iter().next().unwrap();
            let tv = self.mp.get_mut(&val).unwrap();
            tv.remove(&t_idx);
            let mut ch_idx = false;
            if t_idx != l - 1 {
                self.vc.swap(t_idx, l - 1);
                ch_idx = true;
            }
            self.vc.pop();
            if tv.is_empty() {
                self.mp.remove(&val);
            }

            if ch_idx {
                let tv = self.mp.get_mut(&self.vc[t_idx]).unwrap();
                tv.remove(&(l - 1));
                tv.insert(t_idx);
            }

            true
        } else {
            false
        }
    }

    fn get_random(&mut self) -> i32 {
        *self.vc.choose(&mut self.rng).unwrap()
    }
}

struct RandomizedSet {
    mp: HashMap<i32, usize>,
    vc: Vec<i32>,
    rng: ThreadRng,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    fn new() -> Self {
        Self {
            mp: HashMap::new(),
            vc: Vec::new(),
            rng: thread_rng(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if let std::collections::hash_map::Entry::Vacant(e) = self.mp.entry(val) {
            self.vc.push(val);
            e.insert(self.vc.len() - 1);
            true
        } else {
            false
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        if !self.mp.contains_key(&val) {
            false
        } else {
            let l = self.vc.len();
            let rm_node = self.mp.remove(&val).unwrap();
            if rm_node == l - 1 {
                self.vc.pop();
            } else {
                self.vc.swap(rm_node, l - 1);
                self.vc.pop();
                self.mp.insert(self.vc[rm_node], rm_node);
            }
            true
        }
    }

    fn get_random(&mut self) -> i32 {
        *self.vc.choose(&mut self.rng).unwrap()
    }
}
