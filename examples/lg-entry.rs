/// 洛谷-入门与面试题库，随缘做
use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    b2025::in_out();
}

mod b2025 {
    use super::*;

    pub fn in_out() {
        let mut bw = BufWriter::new(io::stdout().lock());
        bw.write_fmt(format_args!("{}\n", diamond(7))).unwrap();
        bw.flush().unwrap()
    }

    fn diamond(diagonal: usize) -> String {
        let mut star = 1;
        let mut offset = diagonal / 2;
        let mut res = String::new();
        let mut flag = false;
        for _ in 0..diagonal {
            res.push_str(&format!("{}{}\n", " ".repeat(offset), "*".repeat(star)));
            if offset == 0 {
                flag = true
            }
            if !flag {
                offset -= 1;
                star = star + 2;
            } else if star >= 2 {
                offset += 1;
                star = star - 2;
            }
        }
        res
    }
}

mod b2023 {

    use super::*;

    pub fn in_out() {
        let mut br = BufReader::new(io::stdin().lock());
        let mut bw = BufWriter::new(io::stdout().lock());
        let mut buf = String::new();
        for i in 0..4 {
            br.read_line(&mut buf).unwrap();
            match i {
                0 => {
                    let c: char = buf.trim().parse().unwrap();
                    bw.write_fmt(format_args!("{} ", c)).unwrap()
                }
                1 => {
                    let i: i32 = buf.trim().parse().unwrap();
                    bw.write_fmt(format_args!("{} ", i)).unwrap()
                }
                2 => {
                    let f: f32 = buf.trim().parse().unwrap();
                    bw.write_fmt(format_args!("{:.6} ", f)).unwrap()
                }
                3 => {
                    let f: f64 = buf.trim().parse().unwrap();
                    bw.write_fmt(format_args!("{:.6} ", f)).unwrap()
                }
                _ => unreachable!(),
            }
            buf.clear();
        }
        bw.write_fmt(format_args!("\n")).unwrap();
        bw.flush().unwrap()
    }
}

mod b2021_2022 {

    use super::*;

    pub fn in_out() {
        let mut br = BufReader::new(io::stdin().lock());
        let mut bw = BufWriter::new(io::stdout().lock());
        let mut buf = String::new();
        br.read_line(&mut buf).unwrap();
        // let a: f32 = buf.trim().parse().unwrap();
        // bw.write_fmt(format_args!("{:.3}\n", a)).unwrap();
        // b2022
        let a: f64 = buf.trim().parse().unwrap();
        bw.write_fmt(format_args!("{:.12}\n", a)).unwrap();
        bw.flush().unwrap()
    }
}

mod b2020 {

    use super::*;

    pub fn in_out() {
        let mut br = BufReader::new(io::stdin().lock());
        let mut bw = BufWriter::new(io::stdout().lock());
        let mut buf = String::new();
        br.read_line(&mut buf).unwrap();
        let mut old: Vec<i32> = buf
            .trim()
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();
        let new = candy(&mut old);
        for &i in new.0 {
            bw.write_fmt(format_args!("{} ", i)).unwrap();
        }
        bw.write_fmt(format_args!("\n{}\n", new.1)).unwrap();
        bw.flush().unwrap()
    }

    fn candy(old: &mut [i32]) -> (&[i32], i32) {
        let mut eat = 0;
        let n = old.len();
        for i in 0..n {
            eat += old[i] % 3;
            let s = old[i] / 3;
            old[(i + 1) % n] += s;
            old[i] = s;
            old[if i == 0 { n - 1 } else { i - 1 }] += s;
        }
        (old, eat)
    }
}

mod b2019 {

    use super::*;

    pub fn in_out() {
        let mut br = BufReader::new(io::stdin().lock());
        let mut bw = BufWriter::new(io::stdout().lock());
        let mut buf = String::new();
        br.read_line(&mut buf).unwrap();
        let a: i32 = buf.trim().parse().unwrap();
        bw.write_fmt(format_args!("{}\n", if a == 0 { 0 } else { 1 }))
            .unwrap();
        bw.flush().unwrap()
    }
}

mod b2018 {

    use super::*;

    pub fn in_out() {
        let mut br = BufReader::new(io::stdin().lock());
        let mut bw = BufWriter::new(io::stdout().lock());
        let mut buf = String::new();
        br.read_line(&mut buf).unwrap();
        let a: u8 = buf.trim().parse().unwrap();
        bw.write_fmt(format_args!("{}\n", String::from_utf8(vec![a]).unwrap()))
            .unwrap();
        bw.flush().unwrap()
    }
}

mod b2016 {

    use super::*;

    pub fn in_out() {
        let mut br = BufReader::new(io::stdin().lock());
        let mut bw = BufWriter::new(io::stdout().lock());
        let mut buf = String::new();
        br.read_line(&mut buf).unwrap();
        let a: f64 = buf.trim().parse().unwrap();
        bw.write_fmt(format_args!("{}\n", a as i64)).unwrap();
        bw.flush().unwrap()
    }
}

mod b2015 {

    use super::*;

    pub fn in_out() {
        let mut br = BufReader::new(io::stdin().lock());
        let mut bw = BufWriter::new(io::stdout().lock());
        let mut buf = String::new();
        br.read_line(&mut buf).unwrap();
        let mut segs = buf.trim().split_whitespace();
        let r1: f64 = segs.next().unwrap().parse().unwrap();
        let r2: f64 = segs.next().unwrap().parse().unwrap();
        bw.write_fmt(format_args!("{:.2}\n", r1 * r2 / (r1 + r2)))
            .unwrap();
        bw.flush().unwrap()
    }
}

mod b2014 {

    use super::*;

    pub fn in_out() {
        let mut br = BufReader::new(io::stdin().lock());
        let mut bw = BufWriter::new(io::stdout().lock());
        let mut buf = String::new();
        br.read_line(&mut buf).unwrap();
        let a: f64 = buf.trim().parse().unwrap();
        bw.write_fmt(format_args!(
            "{:.4} {:.4} {:.4}\n",
            diameter(a),
            circumference(a),
            area(a)
        ))
        .unwrap();
        bw.flush().unwrap()
    }

    const PI: f64 = 3.14159;

    fn diameter(r: f64) -> f64 {
        r * 2.0
    }

    fn circumference(r: f64) -> f64 {
        diameter(r) * PI
    }

    fn area(r: f64) -> f64 {
        r.powf(2.0) * PI
    }
}

mod b2013 {

    use super::*;

    pub fn in_out() {
        let mut br = BufReader::new(io::stdin().lock());
        let mut bw = BufWriter::new(io::stdout().lock());
        let mut buf = String::new();
        br.read_line(&mut buf).unwrap();
        let a: f64 = buf.trim().parse().unwrap();
        bw.write_fmt(format_args!("{:.5}\n", 5.0 * (a - 32.0) / 9.0))
            .unwrap();
        bw.flush().unwrap()
    }
}

mod b2012 {

    use super::*;

    pub fn in_out() {
        let mut br = BufReader::new(io::stdin().lock());
        let mut bw = BufWriter::new(io::stdout().lock());
        let mut buf = String::new();
        br.read_line(&mut buf).unwrap();
        let a: f64 = buf.trim().parse().unwrap();
        buf.clear();
        br.read_line(&mut buf).unwrap();
        let b: f64 = buf.trim().parse().unwrap();
        bw.write_fmt(format_args!("{:.3}%\n", (b / a) * 100_f64))
            .unwrap();
        bw.flush().unwrap()
    }
}

mod b2008_2009_2010_2011 {

    use super::*;

    pub fn in_out() {
        let mut br = BufReader::new(io::stdin().lock());
        let mut bw = BufWriter::new(io::stdout().lock());
        let mut buf = String::new();
        br.read_line(&mut buf).unwrap();
        let mut segs = buf.trim().split_whitespace();
        let a: f64 = segs.next().unwrap().parse().unwrap();
        let b: f64 = segs.next().unwrap().parse().unwrap();
        // let c: i32 = segs.next().unwrap().parse().unwrap();
        // b2008
        // bw.write_fmt(format_args!("{}\n", (a + b) * c)).unwrap();
        // b2009
        // bw.write_fmt(format_args!("{}\n", (a + b) / c)).unwrap();
        // b2010
        // bw.write_fmt(format_args!("{} {}\n", a / b, a % b)).unwrap();
        // b2011
        bw.write_fmt(format_args!("{:.9}\n", a / b)).unwrap();
        bw.flush().unwrap()
    }
}

mod b2006 {

    use super::*;

    pub fn in_out() {
        let mut br = BufReader::new(io::stdin().lock());
        let mut bw = BufWriter::new(io::stdout().lock());
        let mut buf = String::new();
        br.read_line(&mut buf).unwrap();
        let mut segs = buf.trim().split_whitespace();
        let x: f64 = segs.next().unwrap().parse().unwrap();
        let a: f64 = segs.next().unwrap().parse().unwrap();
        let y: f64 = segs.next().unwrap().parse().unwrap();
        let b: f64 = segs.next().unwrap().parse().unwrap();
        bw.write_fmt(format_args!("{:.2}\n", (a * x - b * y) / (a - b)))
            .unwrap();
        bw.flush().unwrap()
    }
}

mod b2005 {

    use super::*;

    pub fn in_out() {
        let mut br = BufReader::new(io::stdin().lock());
        let mut bw = BufWriter::new(io::stdout().lock());
        let mut buf = String::new();
        br.read_line(&mut buf).unwrap();
        bw.write_fmt(format_args!("{}\n", triangle(buf.trim(), 3)))
            .unwrap();
        bw.flush().unwrap()
    }

    fn triangle(c: &str, mut h: usize) -> String {
        let mut r = String::new();
        let mut i = 0;
        while h > 0 {
            r.push_str(&" ".repeat(h - 1));
            r.push_str(&c.repeat((i << 1) + 1));
            r.push('\n');
            h -= 1;
            i += 1;
        }
        r
    }
}

mod b2004 {

    use super::*;

    pub fn in_out() {
        let mut br = BufReader::new(io::stdin().lock());
        let mut bw = BufWriter::new(io::stdout().lock());
        let mut buf = String::new();
        br.read_line(&mut buf).unwrap();
        let mut segs = buf.trim().split_whitespace();
        let a: i32 = segs.next().unwrap().parse().unwrap();
        let b: i32 = segs.next().unwrap().parse().unwrap();
        let c: i32 = segs.next().unwrap().parse().unwrap();
        bw.write_fmt(format_args!("{:>8} {:>8} {:>8}\n", a, b, c))
            .unwrap();
        bw.flush().unwrap()
    }
}

mod b2003 {

    use super::*;

    pub fn in_out() {
        let mut br = BufReader::new(io::stdin().lock());
        let mut bw = BufWriter::new(io::stdout().lock());
        let mut buf = String::new();
        br.read_line(&mut buf).unwrap();
        let mut segs = buf.trim().split_whitespace();
        segs.next().unwrap();
        let b: usize = segs.next().unwrap().parse().unwrap();
        bw.write_fmt(format_args!("{}\n", b)).unwrap();
        bw.flush().unwrap()
    }
}

mod b2002 {

    use super::*;

    pub fn in_out() {
        let mut bw = BufWriter::new(io::stdout().lock());
        bw.write_fmt(format_args!("Hello,World!\n")).unwrap();
        bw.flush().unwrap()
    }
}

mod b2001_2007 {

    use super::*;

    pub fn in_out() {
        let mut br = BufReader::new(io::stdin().lock());
        let mut bw = BufWriter::new(io::stdout().lock());
        let mut buf = String::new();
        br.read_line(&mut buf).unwrap();
        let mut segs = buf.trim().split_whitespace();
        let a: usize = segs.next().unwrap().parse().unwrap();
        let b: usize = segs.next().unwrap().parse().unwrap();
        bw.write_fmt(format_args!("{}\n", a + b)).unwrap();
        bw.flush().unwrap()
    }
}
