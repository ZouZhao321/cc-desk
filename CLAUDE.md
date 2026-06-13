# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概述

CC Desk 是一个 Tauri 2 桌面应用，用于管理 Claude Code 的模型配置。核心功能是读写 `~/.claude/settings.json` 中的模型相关字段（auth_token、base_url、model 以及 haiku/sonnet/opus 三组模型 ID/Name），并支持预设管理（保存/切换/删除配置方案）。

## 技术栈

- **前端**: Vue 3 + TypeScript + Naive UI + UnoCSS
- **后端**: Rust (Tauri 2)
- **构建**: Vite (前端) + Cargo (Rust)
- **包管理**: pnpm
- **代码规范**: ESLint + Prettier (TS/Vue)、cargo fmt + clippy (Rust)
- **Git Hooks**: Husky + lint-staged

## 常用命令

```bash
# 开发（启动 Vite 开发服务器，端口 1420）
pnpm dev

# 构建（类型检查 + Vite 打包）
pnpm build

# Tauri 开发（同时启动前端和 Rust 后端）
pnpm tauri dev

# Tauri 构建（生成桌面安装包）
pnpm tauri build

# 前端 lint
pnpm lint
pnpm lint:fix

# 前端格式化
pnpm format
pnpm format:check

# Rust lint
pnpm lint:rs          # cargo clippy -- -D warnings
pnpm format:rs        # cargo fmt
pnpm format:rs:check  # cargo fmt --check
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

## 架构

### 前端 → Rust 通信

前端通过 `@tauri-apps/api` 的 `invoke()` 调用 Rust 命令。Rust 侧在 `src-tauri/src/commands.rs` 中定义三个 Tauri command：

- `read_model_config` — 从 `~/.claude/settings.json` 读取模型配置
- `write_model_config` — 原子写入配置（先备份为 `.bak`，写 `.tmp` 再 rename）
- `get_settings_path` — 返回 settings.json 路径

### 前端 Composables

- `useSettings` (`src/composables/useSettings.ts`) — 管理当前配置的加载、编辑、保存状态，通过 invoke 与 Rust 交互
- `usePresets` (`src/composables/usePresets.ts`) — 管理预设的 CRUD，通过 `@tauri-apps/plugin-store` 持久化到 `presets.json`

### 数据流

1. 应用启动 → `useSettings.loadConfig()` 调用 `read_model_config` 读取 `~/.claude/settings.json`
2. 用户编辑配置卡片 → `updateField()` 更新响应式状态，标记 `dirty`
3. 用户点击保存 → `useSettings.saveConfig()` 调用 `write_model_config` 写回文件
4. 预设应用 → `usePresets.applyPreset()` 返回预设配置 → `applyConfig()` 覆盖当前配置 → `saveConfig()` 写入文件

### 组件结构

- `App.vue` — 主布局，编排 ConfigCard、PresetList、PresetDialog
- `ConfigCard.vue` — 单个配置字段的展示/编辑卡片，敏感字段（如 token）使用 `maskToken` 脱敏
- `PresetList.vue` — 预设列表，支持应用和删除
- `PresetDialog.vue` — 新建预设的弹窗

### TypeScript 类型

核心类型定义在 `src/types/index.ts`：`ModelConfig`（配置字段）、`Preset`（预设）、`PresetStore`（持久化结构）、`ConfigField`（卡片元数据）。前端 `ModelConfig` 字段与 Rust `ModelConfig` 结构体一一对应。

## 代码风格

- 使用 Tab 缩进（宽度 4）
- 使用单引号，不加分号
- Rust 代码使用 `rustfmt` 默认风格（edition 2021）
- Vue 组件使用 `<script setup lang="ts">` 语法
- ESLint 已关闭 `vue/multi-word-component-names` 规则
