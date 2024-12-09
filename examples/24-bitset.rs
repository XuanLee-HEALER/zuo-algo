use std::mem;

fn main() {
    let mut bs = Bitset::new(10);
    bs.fix(9);
    bs.fix(8);
    bs.unfix(8);
    bs.fix(0);
    bs.fix(1);
    println!("{}", bs.to_string());
    bs.flip();
    println!("{}", bs.to_string());
}

struct Bitset {
    set: Vec<i32>,
    ones: usize,
    zeros: usize,
    reverse: bool,
    size: usize,
}

impl Bitset {
    fn new(size: i32) -> Self {
        Self {
            set: vec![0; ((size + 31) / 32) as usize],
            ones: 0,
            zeros: size as usize,
            reverse: false,
            size: size as usize,
        }
    }

    /// 更新idx为1
    fn fix(&mut self, idx: i32) {
        let set_idx = (idx / 32) as usize;
        let offset = idx % 32;
        if !self.reverse {
            if (self.set[set_idx] >> offset) & 1 == 0 {
                self.set[set_idx] |= 1 << offset;
                self.ones += 1;
                self.zeros -= 1;
            }
        } else if (self.set[set_idx] >> offset) & 1 == 1 {
            self.set[set_idx] ^= 1 << offset;
            self.ones += 1;
            self.zeros -= 1;
        }
    }

    /// 更新idx为0
    fn unfix(&mut self, idx: i32) {
        let set_idx = (idx / 32) as usize;
        let offset = idx % 32;
        if !self.reverse {
            if (self.set[set_idx] >> offset) & 1 == 1 {
                self.set[set_idx] ^= 1 << offset;
                self.ones -= 1;
                self.zeros += 1;
            }
        } else if (self.set[set_idx] >> offset) & 1 == 0 {
            self.set[set_idx] |= 1 << offset;
            self.ones -= 1;
            self.zeros += 1;
        }
    }

    fn flip(&mut self) {
        self.reverse = !self.reverse;
        mem::swap(&mut self.ones, &mut self.zeros)
    }

    /// 每个位都是1
    fn all(&self) -> bool {
        self.ones == self.size
    }

    /// 至少有1个位是1
    fn one(&self) -> bool {
        self.ones >= 1
    }

    /// 1的个数
    fn count(&self) -> i32 {
        self.ones as i32
    }

    fn to_string(&self) -> String {
        let mut res = String::new();
        let mut counter = 0;
        for i in self.set.iter() {
            for j in 0..=31 {
                if !self.reverse {
                    res.push_str(format!("{}", (i >> j) & 1).as_str());
                } else {
                    res.push_str(format!("{}", ((i >> j) & 1) ^ 1).as_str());
                }
                counter += 1;
                if counter >= self.size {
                    break;
                }
            }
        }
        res
    }
}

struct OriBitset {
    set: Vec<i32>,
}

impl OriBitset {
    fn new(n: usize) -> Self {
        Self {
            set: vec![0; (n + 31) / 32],
        }
    }

    /// 添加一个数，就是将它在指定位数上变成1
    fn add(&mut self, num: i32) {
        self.set[(num / 32) as usize] |= 1 << (num % 32);
    }

    /// 移除一个数，就是将它在指定位数上变成0
    fn remove(&mut self, num: i32) {
        self.set[(num / 32) as usize] &= !(1 << (num % 32));
    }

    fn reverse(&mut self, num: i32) {
        self.set[(num / 32) as usize] ^= 1 << (num % 32);
    }

    fn query(&self, num: i32) -> bool {
        (self.set[(num / 32) as usize] >> (num % 32)) & 1 == 1
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use rand::{thread_rng, Rng};

    use crate::OriBitset;

    #[test]
    fn test_bitset() {
        const SET_SIZE: usize = 10000;
        let mut set1 = OriBitset::new(SET_SIZE);
        let mut set2 = HashSet::new();
        let mut rng = thread_rng();
        println!("开始测试");
        for _ in 0..10_000 {
            let cur_num = rng.gen_range(0..SET_SIZE as i32);
            match cur_num {
                n if n < 333 => {
                    set1.add(n);
                    set2.insert(n);
                }
                n if (333..666).contains(&n) => {
                    set1.remove(n);
                    set2.remove(&n);
                }
                n => {
                    set1.reverse(n);
                    if set2.contains(&n) {
                        set2.remove(&n);
                    } else {
                        set2.insert(n);
                    }
                }
            }
        }
        println!("测试结束");
        println!("开始验证");
        for i in 0..SET_SIZE as i32 {
            assert_eq!(set1.query(i), set2.contains(&i), "the test number is {}", i);
        }
        println!("验证结束");
    }
}
