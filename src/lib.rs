//! 这个 lib crate 包括自己实现的数据结构和一些公共函数，在提交具体题目时需要拷贝到源文件中，只是为了公共结构可以在一个地方定义一次

use std::cmp::Ordering;

use rand::{Rng, thread_rng};

/// 固定大小的堆结构，内部使用一个Vec来保存数据
pub struct Heap<T: Copy + PartialEq + Eq + PartialOrd + Ord> {
    arr: Vec<T>,
    p: usize,
    n: usize,
}

impl<T> Heap<T>
where
    T: Copy + PartialEq + Eq + PartialOrd + Ord,
{
    /// 初始化一个指定大小的堆
    pub fn new(n: usize) -> Self {
        Self {
            arr: Vec::with_capacity(n),
            p: 0,
            n,
        }
    }

    fn heap_insert(&mut self, mut idx: usize) {
        while idx > 0 {
            let p_idx = (idx - 1) >> 1;
            match self.arr[idx].cmp(&self.arr[p_idx]) {
                Ordering::Less => {
                    self.arr.swap(idx, p_idx);
                    idx = p_idx
                }
                _ => break,
            }
        }
    }

    fn heapify(&mut self, size: usize) {
        let mut idx = 0;
        while idx < size {
            let l = idx << 1 | 1;
            let r = l + 1;
            if l < size && r < size {
                match (
                    self.arr[idx].cmp(&self.arr[l]),
                    self.arr[idx].cmp(&self.arr[r]),
                ) {
                    (Ordering::Greater, Ordering::Greater) => match self.arr[l].cmp(&self.arr[r]) {
                        Ordering::Less => {
                            self.arr.swap(idx, l);
                            idx = l
                        }
                        Ordering::Greater | Ordering::Equal => {
                            self.arr.swap(idx, r);
                            idx = r
                        }
                    },
                    (Ordering::Greater, _) => {
                        self.arr.swap(idx, l);
                        idx = l
                    }
                    (_, Ordering::Greater) => {
                        self.arr.swap(idx, r);
                        idx = r
                    }
                    _ => break,
                }
            } else if l < size {
                match self.arr[idx].cmp(&self.arr[l]) {
                    Ordering::Greater => {
                        self.arr.swap(idx, l);
                        idx = l
                    }
                    _ => break,
                }
            } else if r < size {
                match self.arr[idx].cmp(&self.arr[r]) {
                    Ordering::Greater => {
                        self.arr.swap(idx, r);
                        idx = r
                    }
                    _ => break,
                }
            } else {
                break;
            }
        }
    }

    /// 向堆中插入一个元素，如果堆中的元素已满，则不做处理
    pub fn insert(&mut self, t: T) {
        if self.p < self.n {
            if self.p == self.arr.len() {
                self.arr.push(t);
            } else {
                self.arr[self.p] = t;
            }
            self.heap_insert(self.p);
            self.p += 1;
        }
    }

    /// 移除堆顶元素
    pub fn remove(&mut self) -> Option<T> {
        if self.p == 0 {
            None
        } else {
            let v = self.arr[0];
            self.arr.swap(0, self.p - 1);
            self.heapify(self.p - 1);
            self.p -= 1;
            Some(v)
        }
    }

    /// 查看堆顶元素
    pub fn peek(&self) -> Option<&T> {
        if self.p == 0 {
            None
        } else {
            Some(&self.arr[0])
        }
    }

    /// 查看堆是否为空
    pub fn is_empty(&self) -> bool {
        self.p == 0
    }
}

pub struct Diagram {
    pub head: Vec<usize>,
    pub next: Vec<usize>,
    pub to: Vec<usize>,
    cnt: usize,
}

impl Diagram {
    pub fn new(n: usize, m: usize) -> Self {
        Self {
            head: vec![0; n + 1],
            next: vec![0; m + 1],
            to: vec![0; m + 1],
            cnt: 1,
        }
    }

    pub fn add_edge(&mut self, p1: usize, p2: usize) {
        self.next[self.cnt] = self.head[p1];
        self.head[p1] = self.cnt;
        self.to[self.cnt] = p2;
        self.cnt += 1;
    }
}

// 生成一棵起始节点编号为root，节点数为n的树
fn gen_tree(root: usize, n: usize) -> Option<Vec<(usize, usize)>> {
    if n <= 1 {
        return None;
    }
    let mut res = vec![];
    let n = n - 1; // 剩余可用节点数
    // 1/2 2,1/6,3,1/6,4,1/6 5
    // let mut rng = thread_rng();
    // if rng.gen_ratio(1, 2) {
    let p1 = root + 1;
    let p1n = n / 2;
    if p1n > 0 {
        res.push((root, p1));
        if let Some(mut r1) = gen_tree(p1, p1n) {
            res.append(&mut r1);
        }
    }
    let p2 = p1 + p1n;
    let p2n = n - p1n;
    if p2n > 0 {
        res.push((root, p2));
        if let Some(mut r2) = gen_tree(p2, p2n) {
            res.append(&mut r2);
        }
    }
    // } else {
    //     if rng.gen_ratio(1, 3) {
    //         // 3
    //     } else if rng.gen_ratio(1, 2) {
    //         // 4
    //     } else {
    //         // 5
    //     }
    // }
    Some(res)
}

#[cfg(test)]
mod tests {
    use rand::{Rng, thread_rng};

    use super::*;

    #[test]
    fn test_heap() {
        let mut heap = Heap::new(10);
        let mut rng = thread_rng();
        let mut ori: [u8; 10] = rng.r#gen();
        for v in ori {
            heap.insert(v);
        }
        ori.sort_unstable();
        let mut res = Vec::with_capacity(10);
        let mut ct = 0;
        while !heap.is_empty() {
            assert_eq!(ori[ct], *heap.peek().unwrap());
            ct += 1;
            res.push(heap.remove().unwrap());
        }
        assert_eq!(
            ori.to_vec(),
            res,
            "ori after sorted {ori:?}, heap sort {res:?}"
        )
    }

    #[test]
    fn test_gen_tree() {
        let r = gen_tree(1, 8);
        assert!(r.is_some());
        for line in &r.unwrap() {
            println!("{} {}", line.0, line.1)
        }
    }
}
