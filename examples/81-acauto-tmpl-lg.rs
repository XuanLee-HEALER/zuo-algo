use std::{
    collections::VecDeque,
    io::{self, BufRead, BufReader, BufWriter, Write},
};

const MAXN: usize = 200_001;
const CHAR_LEN: usize = 26;

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();
    buf.clear();
    let mut ac = AC::new(MAXN);
    for _ in 0..n {
        br.read_line(&mut buf).unwrap();
        let line = buf.trim();
        ac.insert(line);
        buf.clear();
    }
    let mut graph = Graph::new(MAXN);
    ac.set_fail(&mut graph);
    br.read_line(&mut buf).unwrap();
    let article = buf.trim();
    let mut freq = vec![0; MAXN];
    ac.read_article(article, &mut freq);
    // ac.collect(&graph, &mut freq, 0);
    ac.collect_1(&graph, &mut freq);
    for v in ac.end {
        bw.write_fmt(format_args!("{}\n", freq[v])).unwrap();
    }

    bw.flush().unwrap();
}

fn u8_to_usize(u: u8) -> usize {
    (u - b'a') as usize
}

struct Graph {
    ct: usize,
    head: Vec<usize>,
    next: Vec<usize>,
    to: Vec<usize>,
}

impl Graph {
    fn new(n: usize) -> Self {
        Self {
            ct: 0,
            head: vec![0; n],
            next: vec![0; n],
            to: vec![0; n],
        }
    }

    fn add_edge(&mut self, p1: usize, p2: usize) {
        self.ct += 1;
        self.next[self.ct] = self.head[p1];
        self.head[p1] = self.ct;
        self.to[self.ct] = p2;
    }
}

struct AC {
    ct: usize,
    tree: Vec<[usize; CHAR_LEN]>,
    end: Vec<usize>,
    fail: Vec<usize>,
}

impl AC {
    fn new(n: usize) -> Self {
        Self {
            ct: 0,
            // trie树，行是节点总数，列是字符种类
            tree: vec![[0; CHAR_LEN]; n],
            // 每个目标字符串的结尾节点
            end: Vec::new(),
            // fail指针，默认为0
            fail: vec![0; n],
        }
    }

    // 插入一条目标字符串，建立trie
    fn insert(&mut self, s: &str) {
        // 从0开始查找
        let mut cur = 0;
        for &b in s.as_bytes() {
            // 每个字符先转换成列的索引
            let idx = u8_to_usize(b);
            if self.tree[cur][idx] == 0 {
                // 没有此字符的边，生成一个节点
                self.ct += 1;
                // 将新节点赋值给当前节点的路径
                self.tree[cur][idx] = self.ct
            }
            // 移动节点到下一个位置
            cur = self.tree[cur][idx]
        }
        // 当前字符串的最后一个位置（节点），end的索引就是添加字符串的位置
        self.end.push(cur);
    }

    fn set_fail(&mut self, graph: &mut Graph) {
        // 需要借助一个队列进行bfs
        let mut q = VecDeque::new();
        // 将第一个节点的所有子节点放入队列
        for u in self.tree[0] {
            // 表示有路径的点
            if u != 0 {
                q.push_back(u);
            }
        }
        while let Some(p) = q.pop_front() {
            // 弹出一个点，就要为它的子节点设置fail指针，并且额外设置直通表
            for b in 0..CHAR_LEN {
                // 遍历当前的每条路径
                if self.tree[p][b] == 0 {
                    // 如果没有这条路，那么使用自己的fail指针指向这条路的那个节点
                    self.tree[p][b] = self.tree[self.fail[p]][b]
                } else {
                    // 如果有这条路，设置该节点的fail指针
                    // fail指针的值就是自己的fail指针相同路径的那个点
                    self.fail[self.tree[p][b]] = self.tree[self.fail[p]][b];
                    // 并且把这个存在的子节点加入队列
                    q.push_back(self.tree[p][b]);
                }
            }
            // 将fail指针的关系加入到图中，方向是弹出的点的fail指针指向弹出的点
            graph.add_edge(self.fail[p], p);
        }
    }

    fn read_article(&self, s: &str, times: &mut [usize]) {
        // 从trie的0节点开始遍历文章
        let mut cur = 0;
        for &b in s.as_bytes() {
            // 对于文章中每个字符先找到索引
            let idx = u8_to_usize(b);
            // 可以不判断，0上的累计值没用
            // if self.tree[cur][idx] != 0 {
            // 如果有这条路，那么对应的节点要加1
            times[self.tree[cur][idx]] += 1;
            // }
            // 无论有没有路，直接跳到对应的那个节点，因为设置fail指针的时候trie已经变成了直通表
            cur = self.tree[cur][idx]
        }
    }

    fn collect(&self, graph: &Graph, times: &mut [usize], v: usize) {
        let mut nxt = graph.head[v];
        while nxt != 0 {
            Self::collect(&self, graph, times, graph.to[nxt]);
            times[v] += times[graph.to[nxt]];
            nxt = graph.next[nxt]
        }
    }

    fn collect_1(&self, graph: &Graph, times: &mut [usize]) {
        let mut stack = Vec::new();
        stack.push((0_usize, false));
        while let Some((node, visit)) = stack.last_mut() {
            if !*visit {
                *visit = true;
                let mut nxt = graph.head[*node];
                while nxt != 0 {
                    stack.push((graph.to[nxt], false));
                    nxt = graph.next[nxt]
                }
            } else {
                let mut nxt = graph.head[*node];
                while nxt != 0 {
                    times[*node] += times[graph.to[nxt]];
                    nxt = graph.next[nxt]
                }
                stack.pop();
            }
        }
    }
}
