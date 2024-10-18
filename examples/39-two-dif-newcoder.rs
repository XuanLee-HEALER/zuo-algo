use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdin = stdin.lock();
    let stdout = stdout.lock();
    let mut br = BufReader::new(stdin);
    let mut bw = BufWriter::new(stdout);
    let mut grid = vec![vec![0; 1_005]; 1_005];
    let mut buf = String::new();
    let mut n = 0;
    let mut cur_n = 0;
    let mut m = 0;
    let mut q = 0;
    let mut head = false;
    while let Ok(sz) = br.read_line(&mut buf) {
        if sz > 0 {
            if !head {
                let mut segs = buf.trim().split(" ");
                n = segs.next().unwrap().parse().unwrap();
                m = segs.next().unwrap().parse().unwrap();
                q = segs.next().unwrap().parse().unwrap();
                head = true
            } else if n > 0 {
                let mut segs = buf.trim().split(" ");
                for j in 0..m {
                    set(
                        &mut grid,
                        cur_n + 1,
                        j + 1,
                        cur_n + 1,
                        j + 1,
                        segs.next().unwrap().parse().unwrap(),
                    );
                }
                cur_n += 1;
                n -= 1;
            } else if q > 0 {
                let mut segs = buf.trim().split(" ");
                let r1 = segs.next().unwrap().parse().unwrap();
                let c1 = segs.next().unwrap().parse().unwrap();
                let r2 = segs.next().unwrap().parse().unwrap();
                let c2 = segs.next().unwrap().parse().unwrap();
                let v = segs.next().unwrap().parse().unwrap();
                set(&mut grid, r1, c1, r2, c2, v);
                q -= 1;

                if q == 0 {
                    build(&mut grid, cur_n, m);
                    for sub in grid[1..=cur_n].iter() {
                        for e in sub[1..m].iter() {
                            bw.write_fmt(format_args!("{} ", *e)).unwrap();
                        }
                        bw.write_fmt(format_args!("{}\n", sub[m])).unwrap();
                    }
                    cur_n = 0;
                    head = false;
                    for sub in grid.iter_mut() {
                        for e in sub.iter_mut() {
                            *e = 0;
                        }
                    }
                }
            }
        } else {
            break;
        }
        buf.clear();
    }

    bw.flush().unwrap();
}

fn set(arr: &mut [Vec<i64>], r1: usize, c1: usize, r2: usize, c2: usize, v: i64) {
    arr[r1][c1] += v;
    arr[r2 + 1][c1] -= v;
    arr[r1][c2 + 1] -= v;
    arr[r2 + 1][c2 + 1] += v;
}

fn build(arr: &mut [Vec<i64>], row: usize, col: usize) {
    for i in 1..=row {
        for j in 1..=col {
            arr[i][j] += arr[i - 1][j] + arr[i][j - 1] - arr[i - 1][j - 1]
        }
    }
}
