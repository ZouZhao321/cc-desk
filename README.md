# CC Desk

CC Desk 是一个基于 Tauri 2 的桌面应用，用于管理 Claude Code 的模型配置。

## 技术栈

| 层        | 技术                                                  |
| --------- | ----------------------------------------------------- |
| 前端      | Vue 3 + TypeScript + Naive UI + UnoCSS                |
| 后端      | Rust (Tauri 2)                                        |
| 构建      | Vite + Cargo                                          |
| 包管理    | pnpm                                                  |
| 代码规范  | ESLint + Prettier (TS/Vue)、cargo fmt + clippy (Rust) |
| Git Hooks | Husky + lint-staged                                   |

## 快速开始

### 环境要求

- Node.js >= 18
- pnpm
- Rust (via [rustup](https://rustup.rs/))
- Tauri 2 系统依赖（参见 [Tauri Prerequisites](https://v2.tauri.app/start/prerequisites/)）

### 安装依赖

```bash
pnpm install
```

### 开发

```bash
# 启动前端开发服务器（端口 1420）
pnpm dev

# 启动 Tauri 开发模式（前端 + Rust 后端）
pnpm tauri dev
```

### 构建

```bash
# 前端构建（类型检查 + 打包）
pnpm build

# 生成桌面安装包
pnpm tauri build
```

### 目录结构

```
src/
  components/    # Vue 组件
  composables/   # 组合式函数（useSettings、usePresets）
  types/         # TypeScript 类型定义
  utils/         # 工具函数

src-tauri/src/   # Rust 后端（Tauri 命令、应用入口）
```

## 许可证

[AGPL-3.0](LICENSE)
