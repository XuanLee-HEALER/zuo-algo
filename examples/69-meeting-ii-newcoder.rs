use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    io::{self, BufRead, BufReader, BufWriter, Write},
};

fn main() {
    let mut br = BufReader::new(io::stdin());
    let mut bw = BufWriter::new(io::stdout());
    let mut buf = String::new();
    let mut head = false;
    let mut n = 0;
    let mut counter = 0;
    let mut meetings = vec![];
    while let Ok(sz) = br.read_line(&mut buf) {
        if sz > 0 {
            if !head {
                n = buf.trim().parse().unwrap();
                head = true;
            } else {
                let mut segs = buf.trim().split(" ");
                let beg = segs.next().unwrap().parse().unwrap();
                let end = segs.next().unwrap().parse().unwrap();
                meetings.push((beg, end));
                counter += 1;
                if counter == n {
                    bw.write_fmt(format_args!("{}\n", compute(meetings)))
                        .unwrap();
                    break;
                }
            }
        } else {
            break;
        }
        buf.clear();
    }
    bw.flush().unwrap()
}

fn compute(mut meetings: Vec<(i32, i32)>) -> i32 {
    meetings.sort_by(|a, b| a.0.cmp(&b.0));
    let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    let mut res = 0;
    for &(beg, end) in &meetings {
        if !min_heap.is_empty() && beg >= min_heap.peek().unwrap().0 {
            min_heap.pop();
            min_heap.push(Reverse(end));
        } else {
            min_heap.push(Reverse(end));
        }
        res = res.max(min_heap.len() as i32)
    }
    res
}
