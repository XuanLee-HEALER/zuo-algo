use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();
    let mut cnts = vec![0; n + 1];
    buf.clear();
    br.read_line(&mut buf).unwrap();
    buf.trim()
        .split(" ")
        .for_each(|s| cnts[s.parse::<usize>().unwrap()] += 1);
    bw.write_fmt(format_args!("{}", cnts.iter().filter(|&v| *v > 0).count()))
        .unwrap();
    bw.flush().unwrap()
}
