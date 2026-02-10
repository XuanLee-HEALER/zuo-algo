# zuo-algo

左程云算法课程的 Rust 实现笔记。

## 项目结构

- `examples/` — 按讲次编号组织的算法练习（222 个文件），如 `98-delete-edge-lg.rs`
- `src/lib.rs` — 公共数据结构（Heap 等），提交题目时需拷贝到源文件
- `src/main.rs` — 主入口
- `c/` — 部分 C/C++ 实现

## 运行方式

```bash
cargo run --example <name>       # 运行指定 example（不含 .rs 后缀）
cargo build --examples           # 编译所有 examples
```

## 文件命名规范

- 格式：`{讲次编号}-{主题}.rs`，如 `97-lca-ontree-lg.rs`
- 洛谷题目后缀 `-lg`，牛客后缀 `-newcoder`
- LeetCode 题目放 `src/` 目录，命名 `lc{题号}.rs`
- 同一讲多个文件用不同主题名区分

## 提交规范

- 讲次提交：`第X讲，主题`（中文数字 + 中文逗号）
- 刷题提交：`小学生继续🐛`
