---
name: solve
description: 在 src/ 目录下新建题目文件并注册 mod，支持 lc 和 acm 两种代码风格
disable-model-invocation: true
---

# 做题

在 `src/` 目录下创建新的题目文件，并在 `src/main.rs` 中添加 `mod` 声明。

## 参数

- **文件名**：不含 `.rs` 后缀，如 `lc42`、`cf1900a`
- **代码风格**：`lc`（LeetCode）或 `acm`（Codeforces / 洛谷等需要自己处理输入输出的题目）

## 操作步骤

1. 在 `src/` 下创建 `{文件名}.rs`
2. 根据代码风格填入对应模板
3. 在 `src/main.rs` 的 mod 声明区域添加 `mod {文件名};`（按已有 mod 的位置追加即可）

## LeetCode 模板（lc）

```rust
struct Solution;

impl Solution {
    // TODO: 实现题目要求的方法
}

#[cfg(test)]
mod test_{文件名} {
    use super::*;

    #[test]
    fn test() {
        // TODO: 添加测试用例
    }
}
```

特点：
- 不需要 `main` 函数，通过 `#[cfg(test)]` 测试模块验证
- 运行方式：`cargo test {文件名}`

## ACM 模板（acm）

```rust
use std::io::{self, BufRead, BufReader, BufWriter, Write};

#[allow(dead_code)]
fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();

    br.read_line(&mut buf).unwrap();
    // TODO: 解析输入并实现算法

    bw.flush().unwrap();
}
```

特点：
- 使用 `BufReader` / `BufWriter` 高效处理 IO
- `main` 函数加 `#[allow(dead_code)]` 避免警告（因为 `src/main.rs` 中已有 main）
- 提交到 OJ 时直接拷贝整个文件内容
