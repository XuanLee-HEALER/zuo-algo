use std::ops::{BitAnd, Neg};

fn main() {
    let mut na = NumArray::new(vec![1, 3, 5]);
    println!("{}", na.sum_range(0, 2));
    na.update(1, 2);
    println!("{}", na.sum_range(0, 2));

    let mut nm = NumMatrix::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
    println!("matrix: {:?}", nm.arr);
    println!("{}", nm.sum_region(0, 0, 2, 2));
    println!("{}", nm.sum_region(1, 1, 2, 2));
    nm.update(2, 1, 1);
    println!("{}", nm.sum_region(0, 0, 2, 2));
    println!("{}", nm.sum_region(1, 1, 2, 2));
}

struct NumMatrix {
    n: usize,
    m: usize,
    arr: Vec<Vec<i32>>,
    nums: Vec<Vec<i32>>,
}

impl NumMatrix {
    fn add(&mut self, i: usize, j: usize, v: i32) {
        let mut i = i as i32;
        while i <= self.n as i32 {
            let mut j = j as i32;
            while j <= self.m as i32 {
                self.arr[i as usize][j as usize] += v;
                j += bk(j)
            }
            i += bk(i)
        }
    }

    fn sum(&self, i: usize, j: usize) -> i32 {
        let mut res = 0;
        let mut i = i as i32;
        while i > 0 {
            let mut j = j as i32;
            while j > 0 {
                res += self.arr[i as usize][j as usize];
                j -= bk(j)
            }
            i -= bk(i)
        }
        res
    }

    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let n = matrix.len();
        let m = matrix[0].len();
        let arr = vec![vec![0; m + 1]; n + 1];
        let mut o = Self {
            n,
            m,
            arr,
            nums: vec![],
        };
        for (i, sub) in matrix.iter().enumerate() {
            for (j, &v) in sub.iter().enumerate() {
                o.add(i + 1, j + 1, v);
            }
        }
        o.nums = matrix;
        o
    }

    fn update(&mut self, x: usize, y: usize, v: i32) {
        self.add(x + 1, y + 1, v - self.nums[x][y]);
        self.nums[x][y] = v
    }

    fn sum_region(&self, a: usize, b: usize, c: usize, d: usize) -> i32 {
        self.sum(c + 1, d + 1) - self.sum(c + 1, b) - self.sum(a, d + 1) + self.sum(a, b)
    }
}

struct NumArray {
    n: usize,
    arr: Vec<i32>,
    nums: Vec<i32>,
}

fn bk<T: Neg<Output = T> + BitAnd<Output = T> + Copy>(v: T) -> T {
    v & (-v)
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn add(&mut self, i: usize, v: i32) {
        let mut i = i as i32;
        while i <= self.n as i32 {
            self.arr[i as usize] += v;
            i += bk(i)
        }
    }

    fn sum_t(&self, i: usize) -> i32 {
        let mut res = 0;
        let mut i = i as i32;
        while i > 0 {
            res += self.arr[i as usize];
            i -= bk(i)
        }
        res
    }

    fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let t = vec![0; n + 1];
        let mut o = Self {
            n,
            arr: t,
            nums: vec![],
        };
        for (i, &v) in nums.iter().enumerate() {
            o.add(i + 1, v);
        }
        o.nums = nums;
        o
    }

    fn update(&mut self, index: i32, val: i32) {
        self.add(index as usize + 1, val - self.nums[index as usize]);
        self.nums[index as usize] = val;
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.sum_t(right as usize + 1) - self.sum_t(left as usize)
    }
}
