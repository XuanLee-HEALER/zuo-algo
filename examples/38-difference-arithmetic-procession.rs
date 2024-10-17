use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin());
    let mut bw = BufWriter::new(io::stdout());

    let mut buf = String::new();
    let mut problem_arr = vec![0; 10_000_003];
    let mut head: bool = false;
    let mut counter = 0;
    let mut n = 0;
    let mut params = Vec::new();
    while let Ok(sz) = br.read_line(&mut buf) {
        if sz > 0 {
            if !head {
                let headline: Vec<&str> = buf.trim().split(" ").collect();
                n = headline[0].parse().unwrap();
                counter = headline[1].parse().unwrap();

                head = true
            } else if counter > 0 {
                let p: Vec<&str> = buf.trim().split(" ").collect();
                params.push(ParamGroup(
                    p[0].parse().unwrap(),
                    p[1].parse().unwrap(),
                    p[2].parse().unwrap(),
                    p[3].parse().unwrap(),
                ));
                counter -= 1;

                if counter == 0 {
                    let ans = difference_arithmetic_procession_mode(&mut problem_arr, &params, n);
                    bw.write_fmt(format_args!("{} {}\n", ans.0, ans.1)).unwrap();
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

struct ParamGroup(usize, usize, i32, i32);

fn difference_arithmetic_procession_mode(
    dif_arr: &mut [i32],
    pg: &[ParamGroup],
    n: usize,
) -> (i32, i32) {
    // an = a1 + (n-1)*d
    // d  = (an - a1)/(n-1)
    // n = (end-beg)+1
    // d = (an-a1)/(end-beg)
    for p in pg {
        let &ParamGroup(l, r, beg, end) = p;
        let d = (end - beg) / ((r - l) as i32);
        dif_arr[l] += beg;
        dif_arr[l + 1] += d - beg;
        dif_arr[r + 1] -= d + end;
        dif_arr[r + 2] += end;
    }

    let mut sum = 0;
    for v in dif_arr[1..=n].iter_mut() {
        sum += *v;
        *v = sum;
    }

    sum = 0;
    for v in dif_arr[1..=n].iter_mut() {
        sum += *v;
        *v = sum;
    }

    let mut xor_ans = 0;
    let mut max = i32::MIN;
    for v in dif_arr[1..=n].iter() {
        xor_ans ^= *v;
        max = max.max(*v)
    }

    (xor_ans, max)
}
