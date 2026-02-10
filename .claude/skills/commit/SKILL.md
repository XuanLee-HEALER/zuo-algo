---
name: commit
description: 按照本项目的提交规范创建 git commit
disable-model-invocation: true
---

# 提交规范

本项目的 git commit message 遵循以下格式：

## 常规讲次提交

格式：`第X讲，主题描述`

- X 使用中文数字（如：第一百零六讲）
- 逗号使用中文逗号 `，`
- 主题描述简洁明了

示例：
- `第一百零六讲，树的重心`
- `第一百零五讲，树上倍增和LCA`
- `第九十七讲，线段树原理和代码详解`

## 练习/刷题提交

格式：`小学生继续🐛`

用于非讲次的零散刷题和 bug 修复。

## 操作步骤

1. 运行 `git status` 和 `git diff` 查看变更
2. 运行 `git log --oneline -5` 查看最近的提交风格
3. 根据变更内容判断使用哪种提交格式：
   - 如果是新的一讲内容，使用讲次格式，讲次编号递增
   - 如果是零散练习或修复，使用 `小学生继续🐛`
4. 暂存相关文件（使用具体文件名，不要 `git add -A`）
5. 创建 commit，message 末尾加上 `Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>`
