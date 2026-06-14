# PR 阶段 GitHub CI 设计

## 目标

确保每次 PR 的代码质量，main 分支始终可发布。

## 触发条件

仅在 PR 阶段运行（`opened`、`synchronize`、`reopened`），不触发 push 事件。

## 并发策略

同一 PR 多次 push 只保留最新一次 CI 运行。

## 检查流程

顺序执行，任一失败即停止：

1. **前端代码检查** — ESLint + Prettier 格式校验
2. **Rust 代码检查** — clippy lint + cargo fmt 格式校验
3. **三端构建验证** — Windows / macOS / Linux 并行执行 `tauri build`，仅验证构建成功，不产出安装包

## 依赖关系

检查 → 构建，前端检查 → Rust 检查，串行执行节省资源。
