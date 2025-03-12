use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    let mut head = false;
    let mut n = 0;
    while let Ok(sz) = br.read_line(&mut buf) {
        if sz > 0 {
            if !head {
                n = buf.trim().parse().unwrap();
                head = true;
            } else {
                let tn: i32 = buf.trim().parse().unwrap();
                bw.write_fmt(format_args!(
                    "{}\n",
                    if tn % 6 == 0 {
                        "Roy wins!"
                    } else {
                        "October wins!"
                    }
                ))
                .unwrap();
                n -= 1;
                if n == 0 {
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
