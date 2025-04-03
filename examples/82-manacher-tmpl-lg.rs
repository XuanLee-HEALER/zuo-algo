use std::io::{self, BufRead, BufReader, BufWriter, Write};

const M: u8 = b'#';

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let ms = manacher_s(buf.trim().as_bytes());
    bw.write_fmt(format_args!("{}\n", manacher(&ms))).unwrap();
    bw.flush().unwrap();
}

fn manacher(s: &[u8]) -> usize {
    let n = s.len();
    let mut res = 0;
    let mut p = vec![0; n];
    let (mut c, mut r) = (0, 0);
    let mut i = 0;
    while i < n {
        // 开始遍历字符串
        // 首先任意位置的回文半径是它相对于c的对称点，使用c*2-i计算
        // 前两种情况，也就是新的回文半径至少是以下两个值的最小值
        // 如果r<=i，右边界包不住当前字符，则回文半径至少为1
        let mut len = if r > i { p[c * 2 - i].min(r - i) } else { 1 };
        // i+len是回文半径的结束字符，且要保证两边的索引有效，直接判断覆盖另外两种情况，如果是上面两种情况那么这层循环会失败
        while i + len < n && i >= len && s[i + len] == s[i - len] {
            len += 1;
        }
        // 如果新半径超过了r，那么更新c和r
        if i + len > r {
            r = i + len;
            c = i;
        }
        // 记录新半径
        p[i] = len;
        // 记录最大回文子串
        res = res.max(len);
        i += 1
    }
    // 注意对应关系，需要-1
    res - 1
}

fn manacher_s(s: &[u8]) -> Vec<u8> {
    let mut res = vec![M];
    s.iter().for_each(|&v| {
        res.push(v);
        res.push(M);
    });
    res.push(M);
    res
}
