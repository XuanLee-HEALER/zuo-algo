use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    let mut x_head = false;
    let mut n = 0;
    let mut m = 0;
    let mut val_i = 1;
    let mut cnt = 1;
    let mut head = vec![];
    let mut next = vec![];
    let mut to = vec![];
    let mut val = vec![];
    let mut boss = vec![];
    let mut yes = vec![];
    let mut no = vec![];
    while let Ok(sz) = br.read_line(&mut buf) {
        if sz > 0 {
            if !x_head {
                n = buf.trim().parse().unwrap();
                m = n;
                head = vec![0; n + 1];
                next = vec![0; n];
                to = vec![0; n];
                val = vec![0; n + 1];
                boss = vec![true; n + 1];
                yes = vec![0; n + 1];
                no = vec![0; n + 1];
                x_head = true;
            } else if val_i <= m {
                val[val_i] = buf.trim().parse().unwrap();
                val_i += 1;
            } else {
                let mut segs = buf.trim().split(" ");
                let sub: usize = segs.next().unwrap().parse().unwrap();
                let par: usize = segs.next().unwrap().parse().unwrap();
                boss[sub] = false;
                next[cnt] = head[par];
                head[par] = cnt;
                to[cnt] = sub;
                cnt += 1;
                n -= 1;
                if n == 1 {
                    let mut root = 0;
                    for (i, &e) in boss.iter().enumerate().skip(1) {
                        if e {
                            root = i;
                            break;
                        }
                    }

                    bw.write_fmt(format_args!(
                        "{}\n",
                        compute(root, &head, &next, &to, &val, &mut yes, &mut no)
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

fn compute(
    root: usize,
    head: &[usize],
    next: &[usize],
    to: &[usize],
    val: &[i32],
    yes: &mut [i32],
    no: &mut [i32],
) -> i32 {
    if head[root] == 0 {
        yes[root] = val[root];
        return val[root];
    }
    let mut nx = head[root];
    yes[root] = val[root];
    while nx != 0 {
        let sub = to[nx];
        let sub_res = compute(sub, head, next, to, val, yes, no);
        yes[root] += no[sub];
        no[root] += sub_res;
        nx = next[nx];
    }
    yes[root].max(no[root])
}
