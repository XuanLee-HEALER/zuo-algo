---
name: new-algo
description: 在 examples/ 目录下创建新的算法练习文件
disable-model-invocation: true
---

# 创建新算法文件

在 `examples/` 目录下创建新的算法练习文件。

## 命名规范

文件命名格式：`{编号}-{主题}.rs`

- 编号对应讲次编号（如 `98`）
- 同一讲可以有多个文件，用后缀区分（如 `98-delete-edge-lg.rs`、`98-cows-lg.rs`）
- 主题用英文短横线分隔（kebab-case）
- 来自洛谷的题目后缀加 `-lg`（如 `87-index-tree-sa-rq-lg.rs`）
- 来自牛客的题目后缀加 `-newcoder`
- 来自 LeetCode 的题目放在 `src/` 目录下，命名为 `lc{题号}.rs`

## 文件模板

```rust
use std::io::{self, BufRead, BufWriter, Write};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut lines = stdin.lock().lines();

    // TODO: 实现算法
}
```

## 操作步骤

1. 确认当前最大编号：`ls examples/ | sort -t'-' -k1 -n | tail -5`
2. 根据用户描述确定文件名
3. 使用模板创建文件
4. 如果需要用到 `src/lib.rs` 中的公共结构，添加对应的 `use zuo_algo::*;`
