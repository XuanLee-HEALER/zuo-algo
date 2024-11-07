use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin());
    let mut bw = BufWriter::new(io::stdout());
    let mut buf = String::new();
    let mut head = false;
    let mut n = 0;
    let mut m: i64 = 0;
    while let Ok(sz) = br.read_line(&mut buf) {
        let mut segs = buf.trim().split(" ");
        if sz > 0 {
            if !head {
                n = segs.next().unwrap().parse().unwrap();
                m = segs.next().unwrap().parse().unwrap();
                head = true;
            } else {
                let snack = segs.map(|e| e.parse().unwrap()).collect::<Vec<i64>>();
                let mut left_group = vec![0; 1 << ((n + 1) / 2)];
                let mut right_group = vec![0; 1 << ((n + 1) / 2)];
                let ll = handle(&mut left_group, &snack, m, 0, 0, n >> 1, 0);
                let rl = handle(&mut right_group, &snack, m, 0, n >> 1, n, 0);
                let left_group = &mut left_group[..ll];
                let right_group = &mut right_group[..rl];
                left_group.sort_unstable();
                right_group.sort_unstable();

                if ll == 0 {
                    bw.write_fmt(format_args!("{}\n", rl)).unwrap();
                } else {
                    let mut l = 0;
                    let mut r = rl - 1;
                    let mut ans = 0;
                    while l < ll {
                        if left_group[l] + right_group[r] <= m {
                            ans += r + 1;
                            l += 1;
                        } else if r > 0 {
                            r -= 1;
                        } else {
                            break;
                        }
                    }
                    bw.write_fmt(format_args!("{}\n", ans)).unwrap();
                }

                break;
            }
        } else {
            break;
        }
        buf.clear();
    }
    bw.flush().unwrap();
}

// fn handle(
//     group: &mut [i64],
//     snack: &[i64],
//     packet: i64,
//     sum: i64,
//     begin: usize,
//     end: usize,
//     mut last: usize,
// ) -> usize {
//     if sum > packet {
//         return last;
//     }
//     // 如果begin是0，左边等效于什么都不选，可以减少单独判断
//     if begin == end {
//         group[last] = sum;
//         last += 1;
//     } else {
//         last = handle(group, snack, packet, sum, begin + 1, end, last);
//         last = handle(
//             group,
//             snack,
//             packet,
//             sum + snack[begin],
//             begin + 1,
//             end,
//             last,
//         );
//     }
//     last
// }

fn handle(
    group: &mut [i64],
    snack: &[i64],
    packet: i64,
    sum: i64,
    begin: usize,
    end: usize,
    mut last: usize,
) -> usize {
    if begin == end {
        return last;
    }

    if begin == end - 1 {
        if sum <= packet {
            group[last] = sum;
            last += 1;
        }
        if sum + snack[begin] <= packet {
            group[last] = sum + snack[begin];
            last += 1;
        }
    } else {
        last = handle(group, snack, packet, sum, begin + 1, end, last);
        last = handle(
            group,
            snack,
            packet,
            sum + snack[begin],
            begin + 1,
            end,
            last,
        );
    }
    last
}
