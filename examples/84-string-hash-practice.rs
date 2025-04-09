use std::{
    array,
    collections::{HashMap, HashSet},
};

fn main() {
    println!(
        "res is {}",
        Solution::equal_digit_frequency(String::from("12813829392939291828300102402"))
    );
    println!(
        "res is {}",
        Solution::str_str(String::from("sadbutsad"), String::from("sad"))
    );
    println!(
        "res is {}",
        Solution::repeated_string_match(String::from("abcd"), String::from("cdabcdab"))
    );
    println!(
        "res is {:?}",
        Solution::find_substring(
            String::from("wordgoodgoodgoodbestword"),
            vec!["word".into(), "good".into(), "best".into(), "good".into()]
        )
    );
}

struct Solution;

impl Solution {
    fn equal_digit_frequency(str: String) -> i32 {
        let hasher = StrHasher::new(131);
        let mut ct = [0; 10];
        let idx = |b: u8| -> usize { (b - b'0') as usize };
        let s = str.as_bytes();
        let n = s.len();
        let mut distinct = HashSet::new();
        let mut i = 0;
        while i < n {
            ct.fill(0);
            let (mut max, mut max_kind, mut all_kind) = (0, 0, 0);
            let mut j = i;
            while j < n {
                let ti = idx(s[j]);
                ct[ti] += 1;
                if ct[ti] == 1 {
                    // 这个子串的所有种类+1
                    all_kind += 1;
                }
                if ct[ti] == max {
                    // 新种类也达到了最大词频
                    max_kind += 1;
                } else if ct[ti] > max {
                    max = ct[ti];
                    max_kind = 1;
                }
                if all_kind == max_kind {
                    distinct.insert(hasher.hash(&String::from_utf8_lossy(&s[i..=j])));
                }
                j += 1;
            }
            i += 1;
        }
        distinct.len() as i32
    }

    pub fn str_str(haystack: String, needle: String) -> i32 {
        let n = haystack.len();
        let m = needle.len();
        if n < m {
            return -1;
        }
        let hasher = SubStrHasher::new(&haystack, 131);
        let hasher1 = StrHasher::new(131);
        let hash = hasher1.hash(&needle);
        let mut i = 0;
        while i + m <= haystack.len() {
            if hasher.hash(i, i + m - 1) == hash {
                return i as i32;
            }
            i += 1
        }
        -1
    }

    pub fn repeated_string_match(a: String, b: String) -> i32 {
        let n = a.len();
        let m = b.len();
        let hasher = StrHasher::new(131);
        let hash = hasher.hash(&b);
        let k = (m + n - 1) / n;
        let fa = a.repeat(k + 1);
        let fan = fa.len();
        let sub_hasher = SubStrHasher::new(&fa, 131);
        let mut i = 0;
        while i + m <= fan {
            if sub_hasher.hash(i, i + m - 1) == hash {
                return if i + m - 1 >= n * k {
                    (k + 1) as i32
                } else {
                    k as i32
                };
            }
            i += 1;
        }
        -1
    }

    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let hasher = StrHasher::new(131);
        let hash_vec = words.iter().map(|s| hasher.hash(s)).collect::<Vec<u64>>();
        let mut debt = HashMap::new();
        let mut all_debt = 0;
        hash_vec.iter().for_each(|&v| {
            debt.entry(v).and_modify(|v| *v += 1).or_insert(1);
            all_debt += 1;
        });
        let mut res = vec![];
        let sub_hasher = SubStrHasher::new(&s, 131);
        let wl = words[0].len();
        let m = words.len();
        let al = wl * words.len();
        let n = s.len();
        let mut i = 0;
        let mut cur_debt = HashMap::new();
        let mut cur_all_debt = 0;
        while i < wl && i + al <= n {
            // 每组的起始索引 i，窗口需要 al 长度的字符串，至少要有一个窗口才有比较的意义
            // 每次移动幅度是 wl
            // ti-ti+wl  ti+wl-ti+2*wl
            let mut ti = i;
            for _ in 0..m {
                let cd = sub_hasher.hash(ti, ti + wl - 1);
                cur_debt.entry(cd).and_modify(|v| *v += 1).or_insert(1);
                // 如果是有效还债，则债务加1
                if debt.contains_key(&cd) && cur_debt.get(&cd).unwrap() <= debt.get(&cd).unwrap() {
                    cur_all_debt += 1;
                }
                ti += wl;
            }
            if cur_all_debt == all_debt {
                res.push(i as i32);
            }
            ti = i + wl;
            while ti + al <= n {
                let rem = sub_hasher.hash(ti - wl, ti - 1);
                let add = sub_hasher.hash(ti + al - wl, ti + al - 1);
                // 如果是有效还债，则债务要-1
                if debt.contains_key(&rem) && cur_debt.get(&rem).unwrap() <= debt.get(&rem).unwrap()
                {
                    cur_all_debt -= 1;
                }
                cur_debt.entry(rem).and_modify(|v| *v -= 1);
                if let Some(&v) = cur_debt.get(&rem) {
                    if v == 0 {
                        cur_debt.remove(&rem);
                    }
                }
                cur_debt.entry(add).and_modify(|v| *v += 1).or_insert(1);
                if debt.contains_key(&add) && cur_debt.get(&add).unwrap() <= debt.get(&add).unwrap()
                {
                    cur_all_debt += 1;
                }
                if cur_all_debt == all_debt {
                    res.push(ti as i32);
                }
                ti += wl;
            }
            cur_all_debt = 0;
            cur_debt.clear();
            i += 1;
        }
        res
    }
}

struct SubStrHasher {
    exp: Vec<u64>,
    hash: Vec<u64>,
}

impl SubStrHasher {
    fn new(s: &str, base: u64) -> Self {
        let s = s.as_bytes();
        let n = s.len();
        let val = |u: u8| -> u64 { (u - b'a' + 11) as u64 };
        let mut exp = vec![1_u64; n];
        let mut res = val(s[0]);
        let mut hash = vec![res; n];
        for i in 1..n {
            res = res.wrapping_mul(base).wrapping_add(val(s[i]));
            exp[i] = exp[i - 1].wrapping_mul(base);
            hash[i] = res;
        }
        Self { exp, hash }
    }

    fn hash(&self, l: usize, r: usize) -> u64 {
        if l > 0 {
            self.hash[r].wrapping_sub(self.hash[l - 1].wrapping_mul(self.exp[r - l + 1]))
        } else {
            self.hash[r]
        }
    }
}

struct StrHasher {
    code: [u64; 62],
    base: u64,
}

impl StrHasher {
    fn new(base: u64) -> Self {
        let code: [u64; 62] = array::from_fn(|i| (i + 1) as u64);
        Self { code, base }
    }

    fn hash(&self, s: &str) -> u64 {
        let mut res = 0_u64;
        for &b in s.as_bytes() {
            match b {
                b if b >= b'0' && b <= b'9' => {
                    res = res
                        .wrapping_mul(self.base)
                        .wrapping_add(self.code[(b - b'0') as usize])
                }
                b if b >= b'a' && b <= b'z' => {
                    res = res
                        .wrapping_mul(self.base)
                        .wrapping_add(self.code[(b - b'a' + 10) as usize])
                }
                b if b >= b'A' && b <= b'Z' => {
                    res = res
                        .wrapping_mul(self.base)
                        .wrapping_add(self.code[(b - b'A' + 36) as usize])
                }
                _ => panic!(),
            }
        }
        res
    }
}
