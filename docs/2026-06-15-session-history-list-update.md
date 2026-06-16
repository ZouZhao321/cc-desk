# Session History List View 页面更新计划

## Context

根据 pencil 设计稿 `SessionHistory_List` (DCk7P) 更新现有的会话历史列表页面。当前实现使用 Naive UI 的 `NCollapse` 组件按项目分组展示会话，设计稿要求使用自定义的项目卡片（ProjectCard）和会话项（SessionItem）组件，具有更精细的视觉样式。

## 设计 vs 现状差异

| 区域     | 当前实现                         | 设计稿要求                                        |
| -------- | -------------------------------- | ------------------------------------------------- |
| 顶部栏   | "CC-Desk" 橙色标题 + 文字按钮    | 返回箭头 + "会话管理" 黑色标题 + 刷新图标         |
| 项目分组 | NCollapse 折叠面板               | 自定义 ProjectCard 卡片（灰色头部 + 圆角 + 阴影） |
| 会话项   | 单行布局（时间 + 备注 + 消息数） | 双行布局（元信息行 + 备注行，带图标）             |
| 整体风格 | 基础样式                         | 精细化设计（阴影、圆角、颜色系统）                |

## 实现步骤

### Step 1: 更新顶部栏 + 导航行为

**文件:** `src/components/SessionHistory.vue`, `src/App.vue`

**App.vue 变更：**

- Sessions 页面激活时隐藏顶部 Tab 栏（当前已有 `v-if` 逻辑，需确保 `sessionDetailView` 在列表视图也为 `true`，或新增 `sessionPageActive` 状态）
- 返回按钮 emit `back` 事件，App.vue 切换回 `activePage = 'config'`

**SessionHistory.vue 顶部栏：**

- 左侧：返回箭头图标（lucide `arrow-left`，24x24, `#666666`），点击 emit `back`
- 中间：标题"会话管理"（Inter, 20px, 700, `#111827`）
- 右侧：刷新图标（lucide `refresh-cw`，24x24, `#000000`）
- 移除 "CC-Desk" 文字和文字按钮
- 新增 emit: `'back'`

**Commit:** `feat(session): 更新顶部栏样式，隐藏 Tab 栏改用返回箭头导航`

### Step 2: 创建 ProjectCard 组件

**新文件:** `src/components/ProjectCard.vue`

实现设计稿中的 `ProjectCard_Component` (J10Sex)：

- 外层容器：白色背景，圆角 12px，外阴影，纵向布局，gap 12px
- **ProjectHeader**: 灰色背景 (`#F1F5F9`)，仅顶部圆角 `[12,12,0,0]`，padding 14x20
    - 左侧：文件夹图标 (lucide `folder`, 14x14, `#6366F1`) + 项目名称 (Inter, 14px, 600, `#1E293B`) + 会话数 (Inter, 12px, `#64748B`) + 项目路径 (Inter, 12px)
    - 右侧：chevron 图标 (lucide `chevron-right` 或 `chevron-down`，14x14, `#64748B`)
- **SessionList**: padding 内的会话列表容器，gap 8px

Props:

- `projectName: string`
- `projectPath: string`
- `sessionCount: number`
- `sessions: SessionMeta[]`
- `notes: Record<string, string>`
- `expanded: boolean`

Emits:

- `toggle`
- `session-click: [sessionId: string]`

**Commit:** `feat(component): 添加 ProjectCard 项目分组卡片组件`

### Step 3: 创建 SessionItem 组件

**新文件:** `src/components/SessionItem.vue`

实现设计稿中的 `SessionItem_Component` (xAhpO)：

- 外层：白色背景，圆角 8px，边框 `#E2E8F0` 1px，外阴影 (blur:3, offset:0x1, `#00000008`)，padding 12x16，纵向布局，gap 8px
- **SessionMeta 行**: 横向布局，`justifyContent: space_between`
    - 左侧 MetaLeft: 时间 (Inter, 13px, 500, `#1E293B`) + 分隔符 "·" + 消息数 (Inter, 12px, `#64748B`) + 分隔符 + 最后消息预览 (Inter, 11px, `#94A3B8`)
- **SessionNote 区域**: 横向布局，圆角 6px，背景 `#EEF2FF`，gap 6px，padding 6x10
    - 图标 (lucide `message-square`, 12x12, `#6366F1`) + 备注内容 (Inter, 12px, `#6366F1`)
    - 无备注时：灰色背景 `#F3F4F6`，灰色图标和文字

Props:

- `session: SessionMeta`
- `note: string`

Emits:

- `click: [sessionId: string]`

**Commit:** `feat(component): 添加 SessionItem 会话项组件`

### Step 4: 重构 SessionHistory.vue 主页面

**文件:** `src/components/SessionHistory.vue`

- 移除 `NCollapse` 和 `NCollapseItem` 导入和使用
- 移除 `collapseValue` ref 和相关 watch
- 使用新的 `ProjectCard` 和 `SessionItem` 组件替代
- 保持现有的 `groupedSessions` computed 逻辑
- 更新骨架屏样式以匹配新设计

**Commit:** `refactor(session): 重构 SessionHistory 使用新组件替代 NCollapse`

### Step 5: 更新样式细节

**文件:** `src/components/SessionHistory.vue`, `src/components/ProjectCard.vue`, `src/components/SessionItem.vue`

确保所有样式与设计稿一致：

- 颜色系统：`#111827` (主文字), `#1E293B` (次文字), `#64748B` (辅助文字), `#94A3B8` (分隔符), `#6366F1` (强调色/紫色), `#F1F5F9` (头部背景), `#EEF2FF` (备注背景), `#F8FAFC` (内容区背景)
- 字体：Inter 为主，Geist Mono 用于代码/路径
- 间距：遵循设计稿的 padding/gap 值

**Commit:** `style(session): 微调颜色和间距以匹配设计稿`

## 关键文件

| 文件                                   | 操作                                             |
| -------------------------------------- | ------------------------------------------------ |
| `src/components/SessionHistory.vue`    | 修改 - 更新顶部栏，替换 NCollapse 为 ProjectCard |
| `src/components/ProjectCard.vue`       | 新建 - 项目分组卡片组件                          |
| `src/components/SessionItem.vue`       | 新建 - 单条会话项组件                            |
| `src/components/SessionCard.vue`       | 保留 - 详情页可能仍需要，或后续删除              |
| `src/composables/useSessionHistory.ts` | 不变 - 数据层无需修改                            |
| `src/types/index.ts`                   | 不变 - 类型定义已足够                            |

## 复用现有代码

- `useSessionHistory` composable：继续使用 `sessions`, `getNote`, `loadSessions`, `loadSession`, `clearSession` 等方法
- `SessionMeta` 类型：直接复用
- `formatDate` 函数：从 SessionCard.vue 迁移到 SessionItem.vue
- UnoCSS 工具类：继续使用 Tailwind 风格的原子类

## 验证方案

1. **视觉对比**：运行 `pnpm tauri dev`，切换到 Sessions 页面，对比 pencil 设计稿截图
2. **功能测试**：
    - 点击项目头部可折叠/展开会话列表
    - 点击会话项可进入详情页
    - 刷新按钮可重新加载数据
    - 骨架屏加载状态正常显示
    - 空状态和错误状态正常显示
3. **Lint 检查**：运行 `pnpm lint` 和 `pnpm lint:rs` 确保无报错
4. **测试运行**：运行 `pnpm test` 确保现有测试通过
