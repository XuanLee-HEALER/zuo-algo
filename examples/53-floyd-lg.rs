use std::io::{self, BufRead, BufReader, BufWriter, Write};

const MAXN: usize = 101;
const MAXM: usize = 10_001;

fn main() {
    let mut br = BufReader::new(io::stdin());
    let mut bw = BufWriter::new(io::stdout());
    let mut buf = String::new();

    let mut head = false;
    let mut n = 0;
    let mut m = 0;
    let mut cur_i = 1;
    let mut distances = vec![vec![i32::MAX; MAXN]; MAXN];
    let mut paths = Vec::with_capacity(MAXM);

    while let Ok(sz) = br.read_line(&mut buf) {
        let mut segs = buf.trim().split(" ");
        if sz > 0 {
            if !head {
                n = segs.next().unwrap().parse().unwrap();
                m = segs.next().unwrap().parse().unwrap();
                head = true;
            } else if m > 0 {
                paths.push(segs.next().unwrap().parse::<usize>().unwrap());
                m -= 1;
            } else if n > 0 {
                let mut j = 1;
                for dis in segs {
                    distances[cur_i][j] = dis.parse().unwrap();
                    j += 1;
                }
                cur_i += 1;
                n -= 1;

                // floyd
                if n == 0 {
                    floyd(&mut distances, cur_i - 1);
                    bw.write_fmt(format_args!(
                        "{}\n",
                        paths.windows(2).map(|v| distances[v[0]][v[1]]).sum::<i32>()
                    ))
                    .unwrap();
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

fn floyd(distances: &mut [Vec<i32>], n: usize) {
    for k in 1..=n {
        for i in 1..=n {
            for j in 1..=n {
                if distances[i][k] != i32::MAX
                    && distances[k][j] != i32::MAX
                    && distances[i][k] + distances[k][j] < distances[i][j]
                {
                    distances[i][j] = distances[i][k] + distances[k][j];
                }
            }
        }
    }
}
