use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let n: i32 = buf.trim().parse().unwrap();
    buf.clear();
    for _ in 0..n {
        br.read_line(&mut buf).unwrap();
        buf.clear();
        br.read_line(&mut buf).unwrap();
        let beans = buf
            .trim()
            .split(" ")
            .map(|v| v.parse().unwrap())
            .collect::<Vec<i32>>();
        bw.write_fmt(format_args!("{}\n", compute(&beans))).unwrap();
        buf.clear();
    }
    bw.flush().unwrap();
}

fn compute(beans: &[i32]) -> String {
    let n = beans.len();
    let mut sg = vec![0; n + 1];
    let mut appear = vec![false; 100];
    for i in 1..n {
        appear.fill(false);
        for j in 0..i {
            for k in 0..=j {
                appear[(sg[j] ^ sg[k]) as usize] = true;
            }
        }
        for (ti, &b) in appear.iter().enumerate() {
            if !b {
                sg[i] = ti as i32;
                break;
            }
        }
    }
    let t_sg = beans
        .iter()
        .take(n - 1)
        .enumerate()
        .map(|v| (v.0, *v.1))
        .filter(|v| v.1 % 2 == 1)
        .fold(0, |a, b| a ^ sg[n - 1 - b.0]);
    if t_sg == 0 {
        String::from("-1 -1 -1\n0")
    } else {
        let (mut a, mut b, mut c) = (-1, -1, -1);
        let mut record = false;
        let mut cnt = 0;
        for i in (1..n).rev() {
            for j in (0..i).rev() {
                for k in (0..=j).rev() {
                    if t_sg ^ sg[i] ^ sg[j] ^ sg[k] == 0 {
                        if !record {
                            a = i as i32;
                            b = j as i32;
                            c = k as i32;
                            record = true;
                        }
                        cnt += 1;
                    }
                }
            }
        }
        let base = (n - 1) as i32;
        format!("{} {} {}\n{}", base - a, base - b, base - c, cnt)
    }
}
