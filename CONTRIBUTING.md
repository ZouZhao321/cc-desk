# 贡献指南

## 代码规范

```bash
# 前端
pnpm lint              # ESLint 检查
pnpm lint:fix          # ESLint 自动修复
pnpm format            # Prettier 格式化
pnpm format:check      # Prettier 格式检查

# Rust
pnpm lint:rs           # cargo clippy
pnpm format:rs         # cargo fmt
pnpm format:rs:check   # cargo fmt --check
```

## Git 提交规范

使用 Conventional Commits 格式，描述用中文：

```
<type>(<scope>): <中文描述>
```

常用 type：`feat`、`fix`、`chore`、`docs`、`build`、`refactor`、`style`

示例：

```
feat: 添加 ConfigCard 组件，支持展示/编辑模式和 token 脱敏
fix: 修复 model 字段读写不对称
chore: 清理冗余依赖
```

## Git Hooks

- **pre-commit**: 运行 `lint-staged`，自动对暂存文件执行 eslint --fix、prettier --write、rustfmt
- **pre-push**: 运行 `pnpm lint:rs`（cargo clippy），clippy 警告会阻止推送
