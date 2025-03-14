use std::io::{self, BufRead, BufReader, BufWriter, Write};

struct UnionFind {
    father: Vec<usize>,
    size: Vec<usize>,
    stack: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        let mut father = Vec::with_capacity(size + 1);
        for i in 0..size + 1 {
            father.push(i);
        }
        Self {
            father,
            size: vec![1; size + 1],
            stack: vec![0; size + 1],
        }
    }

    fn find(&mut self, v: i32) -> i32 {
        let mut v = v as usize;
        let mut top = 0;
        while self.father[v] != v {
            self.stack[top] = v;
            top += 1;
            v = self.father[v];
        }

        while top > 0 {
            self.father[self.stack[top - 1]] = v;
            top -= 1;
        }

        v as i32
    }

    fn is_same_set(&mut self, v1: i32, v2: i32) -> bool {
        self.find(v1) == self.find(v2)
    }

    fn union(&mut self, v1: i32, v2: i32) {
        let b_v1 = self.find(v1) as usize;
        let b_v2 = self.find(v2) as usize;
        if b_v1 != b_v2 {
            if self.size[b_v1] >= self.size[b_v2] {
                self.father[b_v2] = b_v1;
                self.size[b_v2] += self.size[b_v1];
            } else {
                self.father[b_v1] = b_v2;
                self.size[b_v1] += self.size[b_v2];
            }
        }
    }
}

/// 第一行两个整数N, M。分别表示数组大小、操作次数
/// 接下来M行，每行有一个整数opt
/// 若opt = 1，后面有两个数x, y，表示查询(x, y)这两个数是否属于同一个集合
/// 若opt = 2，后面有两个数x, y，表示把x, y所在的集合合并在一起
fn main() {
    let mut br = BufReader::new(io::stdin());
    let mut bw = BufWriter::new(io::stdout());
    let mut buf = String::new();
    let mut n = 0;
    let mut m = 0;
    let mut head = false;
    let mut union_find = UnionFind::new(0);
    while let Ok(sz) = br.read_line(&mut buf) {
        if sz > 0 {
            if !head {
                let mut segs = buf.trim().split(" ");
                n = segs.next().unwrap().parse().unwrap();
                m = segs.next().unwrap().parse().unwrap();
                union_find = UnionFind::new(n + 1);
                head = true;
            } else {
                let mut segs = buf.trim().split(" ");
                let op: i32 = segs.next().unwrap().parse().unwrap();
                let e1: i32 = segs.next().unwrap().parse().unwrap();
                let e2: i32 = segs.next().unwrap().parse().unwrap();

                if op == 1 {
                    bw.write_fmt(format_args!(
                        "{}\n",
                        if union_find.is_same_set(e1, e2) {
                            "Yes"
                        } else {
                            "No"
                        }
                    ))
                    .unwrap();
                } else if op == 2 {
                    union_find.union(e1, e2);
                }
                m -= 1;
                if m == 0 {
                    break;
                }
            }
        } else {
            break;
        }
        buf.clear();
    }

    bw.flush().unwrap();
}
