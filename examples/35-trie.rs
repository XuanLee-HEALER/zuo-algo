use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader, BufWriter, Write},
};

fn main() {
    const TREE_LAYER: usize = 150_001;
    let mut br = BufReader::new(io::stdin());
    let mut bw = BufWriter::new(io::stdout());
    let mut line = String::new();
    let mut problem = false;
    let mut op_num: usize = 0;
    let mut trie = TrieArr::new(TREE_LAYER);
    while let Ok(sz) = br.read_line(&mut line) {
        let handle_str = line.trim();
        if sz > 0 {
            if !problem {
                op_num = handle_str.parse().unwrap();
                problem = true;
            } else {
                let op = handle_str.split(" ").collect::<Vec<&str>>();
                match (op[0], op[1]) {
                    ("1", add) => trie.insert(add),
                    ("2", del) => trie.delete(del),
                    ("3", search) => {
                        if trie.search(search) > 0 {
                            bw.write_all(b"YES\n").unwrap();
                        } else {
                            bw.write_all(b"NO\n").unwrap();
                        }
                    }
                    ("4", prefix) => bw
                        .write_fmt(format_args!("{}\n", trie.prefix_number(prefix)))
                        .unwrap(),
                    _ => unreachable!(),
                }
                op_num -= 1;
                if op_num == 0 {
                    problem = false;
                    trie.clear();
                }
            }

            line.clear();
        } else {
            break;
        }
    }
    bw.flush().unwrap();
    // let mut trie = TrieArr::new(10000);
    // let mut rng = thread_rng();
    // let mut random_word = ('a'..='z').collect::<Vec<char>>();
    // let mut ct = HashMap::new();

    // let mut before = Vec::new();
    // let mut after = Vec::new();
    // for i in 0..100 {
    //     random_word.shuffle(&mut rng);
    //     let c_str = random_word.iter().collect::<String>();
    //     if i < 50 {
    //         before.push(c_str.clone());
    //     } else {
    //         after.push(c_str.clone());
    //     }
    //     // println!("current string: \"{}\"", c_str);
    //     trie.insert(&c_str);
    //     *ct.entry(c_str.clone()).or_insert(0) += 1;
    //     assert_eq!(*ct.get(&c_str).unwrap(), trie.search(&c_str));
    //     assert_eq!(
    //         mp_prefix(&ct, &c_str[..=4]),
    //         trie.prefix_number(&c_str[..=4])
    //     );
    // }

    // for del in before.iter() {
    //     if ct.contains_key(del) {
    //         if *ct.get(del).unwrap() == 0 {
    //             ct.remove(del);
    //         } else {
    //             ct.entry(del.clone()).and_modify(|v| {
    //                 if *v > 0 {
    //                     *v -= 1;
    //                 }
    //             });
    //         }
    //     }
    //     trie.delete(del);
    // }

    // before.append(&mut after);
    // for r_str in before {
    //     assert_eq!(*ct.get(&r_str).unwrap_or(&0), trie.search(&r_str))
    // }
}

fn mp_prefix(mp: &HashMap<String, usize>, prefix: &str) -> usize {
    let mut res = 0;
    for k in mp.keys() {
        if k.starts_with(prefix) {
            res += 1;
        }
    }
    res
}

struct TrieArr {
    count: usize,
    nodes: Vec<[usize; 26]>,
    pass: Vec<usize>,
    end: Vec<usize>,
}

impl TrieArr {
    fn new(l: usize) -> Self {
        Self {
            count: 1,
            nodes: vec![[0; 26]; l],
            pass: vec![0; l],
            end: vec![0; l],
        }
    }

    fn insert(&mut self, word: &str) {
        let mut cur = 1;
        // 第一行是空白节点
        self.pass[cur] += 1;
        for c in word.chars() {
            let c_idx = (c as u8 - b'a') as usize;
            if self.nodes[cur][c_idx] == 0 {
                // 如果第n行没有这个字符，就需要再选择新一行作为一层
                self.nodes[cur][c_idx] = self.count + 1;
                self.count += 1;
            }
            cur = self.nodes[cur][c_idx];
            // 每次经过层，节点就要加1
            self.pass[cur] += 1;
        }
        // 最后一层节点就是结尾加1
        self.end[cur] += 1;
    }

    fn delete(&mut self, word: &str) {
        if Self::search(self, word) > 0 {
            let mut cur = 1;
            for c in word.chars() {
                let c_idx = (c as u8 - b'a') as usize;
                // 每次经过层，节点就要减1
                self.pass[self.nodes[cur][c_idx]] -= 1;
                // ⚠️删除节点
                if self.pass[self.nodes[cur][c_idx]] == 0 {
                    self.nodes[cur][c_idx] = 0;
                    return;
                }
                cur = self.nodes[cur][c_idx];
            }
            // 最后一层节点就是结尾减1
            self.end[cur] -= 1;
        }
    }

    fn search(&self, word: &str) -> usize {
        let mut cur = 1;
        for c in word.chars() {
            let c_idx = (c as u8 - b'a') as usize;
            if self.nodes[cur][c_idx] == 0 {
                return 0;
            } else {
                cur = self.nodes[cur][c_idx];
            }
        }
        self.end[cur]
    }

    fn prefix_number(&self, prefix: &str) -> usize {
        let mut cur = 1;
        for c in prefix.chars() {
            let c_idx = (c as u8 - b'a') as usize;
            if self.pass[self.nodes[cur][c_idx]] == 0 {
                return 0;
            } else {
                cur = self.nodes[cur][c_idx];
            }
        }
        self.pass[cur]
    }

    fn clear(&mut self) {
        for i in 0..self.count {
            self.nodes[i] = [0; 26];
            self.pass[i] = 0;
            self.end[i] = 0;
        }
        self.count = 1;
    }
}
