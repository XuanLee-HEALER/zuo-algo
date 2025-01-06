use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    let mut head = false;
    // let mut x_start = false;
    let mut n = 0;
    let mut ai = 0;
    let mut graph = vec![vec![0; 20]; 20];
    // let mut start = vec![0; 19];
    // let mut end = vec![0; 19];
    let mut dp = vec![vec![0; 20]; 1 << 20];
    let mut cur_loc = 0;
    while let Ok(sz) = br.read_line(&mut buf) {
        if sz > 0 {
            if !head {
                n = buf[cur_loc..cur_loc + sz].trim().parse().unwrap();
                head = true;
            } else {
                // if !x_start {
                //     for (i, s) in buf.trim().split(" ").skip(1).enumerate() {
                //         start[i] = s.parse::<i32>().unwrap();
                //     }
                //     x_start = true;
                // } else {
                //     let mut segs = buf.trim().split(" ");
                //     end[ai - 1] = segs.next().unwrap().parse().unwrap();
                //     for (i, s) in segs.enumerate() {
                //         graph[ai - 1][i] = s.parse::<i32>().unwrap();
                //     }
                // }
                for (i, s) in buf[cur_loc..cur_loc + sz].trim().split(" ").enumerate() {
                    graph[ai][i] = s.parse::<i32>().unwrap();
                }
                ai += 1;

                if ai == n as usize {
                    let mut res = i32::MAX;
                    // res = res.min(start[i] + compute(&graph, &end, n - 1, 1 << i, i, &mut dp))
                    res = res.min(compute(&graph, n, 1, 0, &mut dp));
                    bw.write_fmt(format_args!("{}\n", res)).unwrap();
                    break;
                }
            }
            cur_loc += sz
        } else {
            break;
        }
    }
    bw.flush().unwrap();
}

// fn compute(
//     graph: &[Vec<i32>],
//     end: &[i32],
//     n: usize,
//     status: usize,
//     idx: usize,
//     dp: &mut [Vec<i32>],
// ) -> i32 {
fn compute(graph: &[Vec<i32>], n: usize, status: usize, idx: usize, dp: &mut [Vec<i32>]) -> i32 {
    if status == (1 << n) - 1 {
        graph[idx][0]
    } else if dp[status][idx] != 0 {
        dp[status][idx]
    } else {
        let mut res = i32::MAX;
        for i in 0..n {
            if status & (1 << i) == 0 {
                res = res.min(graph[idx][i] + compute(graph, n, status | (1 << i), i, dp));
            }
        }
        dp[status][idx] = res;
        dp[status][idx]
    }
}
