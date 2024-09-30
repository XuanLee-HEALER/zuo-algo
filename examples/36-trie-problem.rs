fn main() {
    let res = Solution1::find_words(
        vec![
            vec!['o', 'a', 'a', 'n'],
            vec!['e', 't', 'a', 'e'],
            vec!['i', 'h', 'k', 'r'],
            vec!['i', 'f', 'l', 'v'],
        ],
        vec![
            "oath".into(),
            "pea".into(),
            "eat".into(),
            "rain".into(),
            "oathi".into(),
            "oathk".into(),
            "oathf".into(),
            "oate".into(),
            "oathii".into(),
            "oathfi".into(),
            "oathfii".into(),
        ],
    );
    println!("{:?}", res)
}

struct Solution1;

impl Solution1 {
    const LEN: usize = 10_001;

    fn insert(
        trie: &mut [[usize; 26]; Self::LEN],
        pass: &mut [usize; Self::LEN],
        end_str: &mut [Option<String>; Self::LEN],
        t_str: &str,
        count: &mut usize,
    ) {
        let mut cur = 1;
        pass[cur] += 1;
        for c in t_str.chars() {
            let idx = (c as u8 - b'a') as usize;
            if trie[cur][idx] == 0 {
                trie[cur][idx] = *count + 1;
                *count += 1;
            }
            cur = trie[cur][idx];
            pass[cur] += 1;
        }
        end_str[cur] = Some(t_str.into());
    }

    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut board = board;
        let mut trie: [[usize; 26]; Self::LEN] = [[0; 26]; Self::LEN];
        let mut pass: [usize; Self::LEN] = [0; Self::LEN];
        let mut end_str: [Option<String>; Self::LEN] = [const { None }; Self::LEN];

        let mut count = 1;
        for word in words.iter() {
            Self::insert(&mut trie, &mut pass, &mut end_str, word, &mut count);
        }

        let mut res = Vec::new();

        for i in 0..board.len() {
            for j in 0..board[i].len() {
                Self::dfs(
                    i,
                    j,
                    &mut board,
                    &trie,
                    &mut pass,
                    &mut end_str,
                    1,
                    &mut res,
                );
            }
        }
        res
    }

    /// 返回值是到当前位置已经搜索到几个字符串
    fn dfs(
        i: usize,
        j: usize,
        board: &mut Vec<Vec<char>>,
        trie: &[[usize; 26]; Self::LEN],
        pass: &mut [usize; Self::LEN],
        end_str: &mut [Option<String>; Self::LEN],
        mut t: usize,
        res: &mut Vec<String>,
    ) -> usize {
        // 剪枝1 如果走过的路，不走
        if i >= board.len() || j >= board[i].len() || board[i][j] == '0' {
            return 0;
        }

        // 剪枝2，如果没走过，但是pass为0，那么不需要走
        // if t == 0可以省略，因为t和pass[t]的关系是，如果t为0，那么pass[t]一定是0
        let cur_c = board[i][j];
        let cur_idx = (cur_c as u8 - b'a') as usize;
        // ⚠️使用经过的值判断
        t = trie[t][cur_idx];
        if pass[t] == 0 {
            return 0;
        }

        // 剪枝3，保存字符串
        let mut collect = 0;

        if end_str[t].is_some() {
            collect += 1;
            res.push(end_str[t].take().unwrap());
        }
        // 还需要继续收集

        board[i][j] = '0';
        if i > 0 {
            collect += Self::dfs(i - 1, j, board, trie, pass, end_str, t, res);
        }
        collect += Self::dfs(i + 1, j, board, trie, pass, end_str, t, res);
        if j > 0 {
            collect += Self::dfs(i, j - 1, board, trie, pass, end_str, t, res);
        }
        collect += Self::dfs(i, j + 1, board, trie, pass, end_str, t, res);

        pass[t] -= collect;
        board[i][j] = cur_c;

        collect
    }
}

struct Trie {
    count: usize,
    trie: [[usize; 12]; 2_000_001],
    pass: [usize; 2_000_001],
}

impl Trie {
    fn new() -> Self {
        Trie {
            count: 1,
            trie: [[0; 12]; 2_000_001],
            pass: [0; 2_000_001],
        }
    }

    fn charToPath(c: char) -> usize {
        match c {
            '0'..='9' => (c as u8 - b'0') as usize,
            '-' => 10,
            '#' => 11,
            _ => unreachable!(),
        }
    }

    fn insert(&mut self, num: &str) {
        let mut cur = 1;
        self.pass[cur] += 1;

        for c in num.chars() {
            let cur_idx = Self::charToPath(c);
            if self.trie[cur][cur_idx] == 0 {
                self.trie[cur][cur_idx] = self.count + 1;
                self.count += 1;
            }
            cur = self.trie[cur][cur_idx];
            self.pass[cur] += 1;
        }
    }

    fn prefix(&self, prefix: &str) -> usize {
        let mut cur = 1;

        for c in prefix.chars() {
            let cur_idx = Self::charToPath(c);
            if self.trie[cur][cur_idx] == 0 {
                return 0;
            }
            cur = self.trie[cur][cur_idx];
        }

        self.pass[cur]
    }

    fn clear(&mut self) {
        for i in 0..self.count {
            self.trie[i] = [0; 12];
            self.pass[i] = 0;
        }
    }
}

struct Solution {}

impl Solution {
    fn new() -> Self {
        Solution {}
    }

    /**
     * 代码中的类名、方法名、参数名已经指定，请勿修改，直接返回方法规定的值即可
     *
     *
     * @param b int整型二维数组
     * @param a int整型二维数组
     * @return int整型一维数组
     */
    pub fn countConsistentKeys(&self, b: Vec<Vec<i32>>, a: Vec<Vec<i32>>) -> Vec<i32> {
        // write code here
        let mut trie = Trie::new();
        let mut buf = String::new();
        for e in a.iter() {
            for idx in 1..e.len() {
                buf.push_str(&(e[idx] - e[idx - 1]).to_string());
                buf.push('#');
            }
            trie.insert(&buf);
            buf.clear();
        }

        let mut res = Vec::new();
        for e in b.iter() {
            for idx in 1..e.len() {
                buf.push_str(&(e[idx] - e[idx - 1]).to_string());
                buf.push('#');
            }
            res.push(trie.prefix(&buf) as i32);
            buf.clear();
        }

        res
    }
}
