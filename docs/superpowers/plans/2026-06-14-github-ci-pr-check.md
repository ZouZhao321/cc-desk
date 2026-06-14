# PR 阶段 GitHub CI 实现计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 创建 GitHub Actions workflow，在 PR 阶段运行前端 lint、Rust lint 和三端 Tauri 构建验证。

**Architecture:** 单个 workflow 文件 `.github/workflows/pr-check.yml`，包含三个 job：frontend-lint → rust-lint → build（matrix 三端）。串行执行，任一失败停止后续。

**Tech Stack:** GitHub Actions, pnpm, Node.js, Rust, Tauri 2

---

## File Structure

- Create: `.github/workflows/pr-check.yml` — PR 阶段 CI workflow

---

### Task 1: 创建 workflow 目录和文件

**Files:**

- Create: `.github/workflows/pr-check.yml`

- [ ] **Step 1: 创建目录**

```bash
mkdir -p .github/workflows
```

- [ ] **Step 2: 创建 workflow 文件**

```yaml
name: PR Check

on:
    pull_request:
        branches: [main]
        types: [opened, synchronize, reopened]

concurrency:
    group: ${{ github.workflow }}-${{ github.head_ref }}
    cancel-in-progress: true

jobs:
    frontend-lint:
        runs-on: ubuntu-latest
        steps:
            - uses: actions/checkout@v4

            - uses: pnpm/action-setup@v4
              with:
                  version: 9

            - uses: actions/setup-node@v4
              with:
                  node-version: 20
                  cache: pnpm

            - run: pnpm install --frozen-lockfile

            - name: Lint
              run: pnpm lint

            - name: Format check
              run: pnpm format:check

    rust-lint:
        needs: frontend-lint
        runs-on: ubuntu-latest
        steps:
            - uses: actions/checkout@v4

            - uses: dtolnay/rust-toolchain@stable
              with:
                  components: clippy, rustfmt

            - uses: Swatinem/rust-cache@v2
              with:
                  workspaces: src-tauri -> target

            - name: Clippy
              run: pnpm lint:rs

            - name: Format check
              run: pnpm format:rs:check

    build:
        needs: rust-lint
        strategy:
            fail-fast: false
            matrix:
                os: [ubuntu-latest, macos-latest, windows-latest]
        runs-on: ${{ matrix.os }}
        steps:
            - uses: actions/checkout@v4

            - uses: pnpm/action-setup@v4
              with:
                  version: 9

            - uses: actions/setup-node@v4
              with:
                  node-version: 20
                  cache: pnpm

            - uses: dtolnay/rust-toolchain@stable

            - uses: Swatinem/rust-cache@v2
              with:
                  workspaces: src-tauri -> target

            - run: pnpm install --frozen-lockfile

            - name: Build Tauri app
              run: pnpm tauri build
```

写入文件：`.github/workflows/pr-check.yml`

- [ ] **Step 3: 验证 YAML 语法**

```bash
cat .github/workflows/pr-check.yml | python3 -c "import sys, yaml; yaml.safe_load(sys.stdin)" 2>&1 || echo "YAML_INVALID"
```

Expected: 无输出（表示 YAML 有效）

- [ ] **Step 4: Commit**

```bash
git add .github/workflows/pr-check.yml
git commit -m "ci: 添加 PR 阶段 GitHub Actions workflow

- 前端 lint + 格式检查
- Rust clippy + 格式检查
- 三端 Tauri 构建验证 (Windows/macOS/Linux)
- 并发控制：同一 PR 只保留最新运行"
```

---

### Task 2: 验证 workflow 触发条件

- [ ] **Step 1: 检查 workflow 文件内容**

确认以下关键配置正确：

```bash
grep -A2 "on:" .github/workflows/pr-check.yml
grep -A3 "concurrency:" .github/workflows/pr-check.yml
grep "needs:" .github/workflows/pr-check.yml
```

Expected:

- `on: pull_request` 仅在 PR 触发
- `concurrency` 配置了 `cancel-in-progress: true`
- `rust-lint` needs `frontend-lint`，`build` needs `rust-lint`

- [ ] **Step 2: 确认无 push 触发**

```bash
grep "push:" .github/workflows/pr-check.yml || echo "NO_PUSH_TRIGGER"
```

Expected: 输出 `NO_PUSH_TRIGGER`
