use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    io::{self, BufRead, BufReader, BufWriter, Write},
};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    buf.clear();
    br.read_line(&mut buf).unwrap();
    let sticks = buf
        .trim()
        .split(" ")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    bw.write_fmt(format_args!("{}\n", compute(sticks))).unwrap();
    bw.flush().unwrap()
}

fn compute(sticks: Vec<i32>) -> i32 {
    let mut min_heap = BinaryHeap::new();
    for stick in sticks {
        min_heap.push(Reverse(stick));
    }
    let mut res = 0;
    while min_heap.len() > 1 {
        let s1 = min_heap.pop().unwrap().0;
        let s2 = min_heap.pop().unwrap().0;
        res += s1 + s2;
        min_heap.push(Reverse(s1 + s2));
    }
    res
}
