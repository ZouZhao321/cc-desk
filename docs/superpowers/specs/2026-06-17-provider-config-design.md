# LLM 供应商配置功能设计文档

> 日期：2026-06-17

## 1. 功能概述

CC Desk 新增"供应商管理"功能，作为可视化配置 Claude Code API 供应商的主界面。核心原理：

- 所有供应商配置保存到 `~/.cc-desk/providers.json`
- 激活某个供应商时，将其模型配置写入 `~/.claude/settings.json` 的 `env` 和顶层 `model` 字段
- 只修改模型相关字段，`settings.json` 中的 `permissions`、`enabledPlugins` 等其他配置完全不动

## 2. 数据模型

### Provider

```typescript
interface Provider {
  id: string
  name: string           // 供应商名称
  notes?: string         // 备注（可选）
  website?: string       // 官网链接（可选）
  apiKey: string         // → env.ANTHROPIC_AUTH_TOKEN
  baseUrl: string        // → env.ANTHROPIC_BASE_URL
  mainModel: string      // → 顶层 model（"haiku" / "sonnet" / "opus"）
  opusModel: string      // → env.ANTHROPIC_DEFAULT_OPUS_MODEL
  sonnetModel: string    // → env.ANTHROPIC_DEFAULT_SONNET_MODEL
  haikuModel: string     // → env.ANTHROPIC_DEFAULT_HAIKU_MODEL
  subAgentModel: string  // → env.ANTHROPIC_REASONING_MODEL（"haiku" / "sonnet" / "opus"）
  reasoningLevel: string // → env.CLAUDE_CODE_EFFORT_LEVEL（"low" / "medium" / "high" / "max" / "xhigh"）
  isActive: boolean
}
```

### ProviderStore

```typescript
interface ProviderStore {
  providers: Provider[]
}
```

## 3. 字段映射关系

| Provider 字段 | settings.json 位置 | 说明 |
|---|---|---|
| `apiKey` | `env.ANTHROPIC_AUTH_TOKEN` | API 密钥 |
| `baseUrl` | `env.ANTHROPIC_BASE_URL` | 请求地址 |
| `mainModel` | 顶层 `model` | 角色名（haiku/sonnet/opus） |
| `opusModel` | `env.ANTHROPIC_DEFAULT_OPUS_MODEL` | Opus 模型 ID |
| `sonnetModel` | `env.ANTHROPIC_DEFAULT_SONNET_MODEL` | Sonnet 模型 ID |
| `haikuModel` | `env.ANTHROPIC_DEFAULT_HAIKU_MODEL` | Haiku 模型 ID |
| `subAgentModel` | `env.ANTHROPIC_REASONING_MODEL` | 子代理模型 |
| `reasoningLevel` | `env.CLAUDE_CODE_EFFORT_LEVEL` | 推理强度 |

派生字段（由 Rust 自动计算，不存储在 Provider 中）：
- `env.ANTHROPIC_MODEL`：根据 `mainModel` 角色名派生（`mainModel="haiku"` → `ANTHROPIC_MODEL=haikuModel`）

不写入 settings.json 的 Provider 字段：
- `name`、`notes`、`website`：仅作为元数据保存在 `providers.json`
- `_MODEL_NAME` 系列字段：不保留

## 4. 文件存储

| 文件 | 路径 | 内容 | 触发时机 |
|---|---|---|---|
| providers.json | `~/.cc-desk/providers.json` | 所有供应商配置 | 任意增删改操作 |
| settings.json | `~/.claude/settings.json` | 仅修改 env 模型字段 + 顶层 model | 激活供应商时 |

`~/.cc-desk/` 目录在应用启动时自动创建（如不存在）。

## 5. 页面结构

### 5.1 Config List Main（主界面）

供应商列表页面，显示所有已保存的供应商卡片。

**顶部栏**：
- 左侧：应用标题 "CC-Desk"、设置按钮、同步按钮
- 右侧：会话按钮、模板按钮、添加按钮（GAMIo）

**供应商卡片**：
- 左侧：拖拽手柄（视觉元素，本期不做排序功能）、供应商 Logo（首字母或图标）
- 中间：供应商名称、官网链接（可点击，调用系统浏览器打开）
- 右侧：操作按钮组
  - 激活按钮：未激活时显示，点击直接激活（无确认弹窗）
  - "使用中"徽标：已激活时显示（绿色）
  - 复制按钮：复制供应商配置
  - 编辑按钮（mhsxV）：跳转编辑页面
  - 删除按钮：删除供应商

**激活状态视觉区分**：
- 当前激活的供应商：左侧蓝色边框（3px）+ 绿色"使用中"徽标
- 未激活的供应商：灰色边框（1px）

### 5.2 Add/Edit Provider Page（添加/编辑共用）

添加和编辑使用同一个页面组件，区别在于编辑模式回填默认值。

**页面头部**：
- 左侧：返回按钮（←）、页面标题（"添加供应商" / "编辑供应商"）
- 右侧：取消按钮

**表单内容**：

#### 基本信息

| 字段 | 必填 | 控件类型 | placeholder |
|---|---|---|---|
| 供应商名称 | 是 | 文本输入 | 例如：DeepSeek |
| 备注 | 否 | 文本输入 | 可选备注信息 |
| 官网链接 | 否 | 文本输入 | https://platform.example.com |

#### API 配置

| 字段 | 必填 | 控件类型 | placeholder |
|---|---|---|---|
| API Key | 是 | 文本输入（密码模式，可切换显示） | sk-xxxxxxxxxxxxxxxx |
| 请求地址 | 是 | 文本输入 | https://api.example.com/anthropic |

#### 模型配置

副标题：配置不同场景使用的模型

| 字段 | 必填 | 控件类型 | 选项/placeholder |
|---|---|---|---|
| 主模型 | 是 | 下拉框 | haiku / sonnet / opus |
| Opus 模型 | 是 | 文本输入 | 模型 ID |
| Sonnet 模型 | 是 | 文本输入 | 模型 ID |
| Haiku 模型 | 是 | 文本输入 | 模型 ID |
| 子代理模型 | 是 | 下拉框 | haiku / sonnet / opus |
| 推理强度 | 是 | 下拉框 | low / medium / high / max / xhigh |

表单布局：模型配置区域采用 2 列网格：
- 第 1 行：主模型、Opus 模型
- 第 2 行：Sonnet 模型、Haiku 模型
- 第 3 行：子代理模型、推理强度

#### 操作栏

- 测试连接按钮：用填写的 API Key 和请求地址发 API 请求验证连通性
- 保存配置按钮：保存供应商配置

## 6. 交互流程

### 6.1 添加供应商

1. 用户点击主界面添加按钮（GAMIo）
2. 跳转到 Add Provider Page（空白表单）
3. 填写表单，所有必填字段校验通过
4. 点击"测试连接"可选验证连通性
5. 点击"保存配置"→ 生成 UUID → 写入 providers.json → 返回主界面

### 6.2 编辑供应商

1. 用户点击供应商卡片的编辑按钮（mhsxV）
2. 跳转到 Add Provider Page（回填该供应商的当前值）
3. 修改字段后点击"保存配置"→ 更新 providers.json → 返回主界面

### 6.3 复制供应商

1. 用户点击供应商卡片的复制按钮
2. 创建该供应商的副本（名称加"(副本)"后缀，生成新 UUID，isActive=false）
3. 写入 providers.json
4. 跳转到 Add Provider Page（编辑模式，回填副本的值）
5. 用户修改后保存

### 6.4 激活供应商

1. 用户点击供应商卡片的"激活"按钮
2. 将该供应商的 isActive 设为 true，其余设为 false
3. 读取该供应商的模型配置，映射写入 settings.json：
   - 顶层 `model` ← `mainModel`
   - `env.ANTHROPIC_AUTH_TOKEN` ← `apiKey`
   - `env.ANTHROPIC_BASE_URL` ← `baseUrl`
   - `env.ANTHROPIC_DEFAULT_OPUS_MODEL` ← `opusModel`
   - `env.ANTHROPIC_DEFAULT_SONNET_MODEL` ← `sonnetModel`
   - `env.ANTHROPIC_DEFAULT_HAIKU_MODEL` ← `haikuModel`
   - `env.ANTHROPIC_REASONING_MODEL` ← `subAgentModel`
   - `env.CLAUDE_CODE_EFFORT_LEVEL` ← `reasoningLevel`
   - `env.ANTHROPIC_MODEL` ← 根据 mainModel 角色派生
4. 保存 providers.json 和 settings.json
5. 主界面刷新，显示新的激活状态

### 6.5 删除供应商

1. 用户点击供应商卡片的删除按钮
2. 弹出确认对话框
3. 确认后从 providers.json 中移除
4. 如果删除的是激活供应商，不清除 settings.json（保留当前配置）

### 6.6 测试连接

1. 用户在表单中点击"测试连接"
2. Rust 后端用填写的 apiKey 和 baseUrl 发送 GET 请求到 `{baseUrl}/v1/models`，携带 `Authorization: Bearer {apiKey}` 头
3. 返回成功/失败状态
4. 前端显示连接结果提示（成功：绿色提示；失败：红色提示 + 错误信息）

## 7. Rust 后端

### 新增依赖

```toml
reqwest = { version = "0.12", features = ["json"] }
tokio = { version = "1", features = ["full"] }
```

`reqwest` 用于测试连接时发送 HTTP 请求，`tokio` 提供异步运行时。

### 新增命令

| 命令 | 参数 | 返回值 | 说明 |
|---|---|---|---|
| `list_providers` | 无 | `Vec<Provider>` | 读取 ~/.cc-desk/providers.json |
| `save_providers` | `providers: Vec<Provider>` | 无 | 写入 ~/.cc-desk/providers.json |
| `activate_provider` | `provider: Provider` | 无 | 将供应商配置写入 settings.json |
| `test_connection` | `api_key: String, base_url: String` | `Result<String>` | GET `{base_url}/v1/models`，返回成功/错误信息 |

### 已有命令（复用）

| 命令 | 说明 |
|---|---|
| `read_model_config` | 读取 settings.json 中的模型配置 |
| `write_model_config` | 写入模型配置到 settings.json（原子写入：备份 → 写 .tmp → rename） |

### 初始化逻辑

应用启动时：
1. 检查 `~/.cc-desk/` 目录是否存在，不存在则创建
2. 检查 `~/.cc-desk/providers.json` 是否存在，不存在则创建空 `{ "providers": [] }`
3. 读取 providers.json，返回给前端

## 8. 前端组件变更

### 新增组件

| 组件 | 路径 | 职责 |
|---|---|---|
| ProviderCard.vue | `src/components/ProviderCard.vue` | 供应商卡片（名称、URL、激活状态、操作按钮） |
| ProviderForm.vue | `src/components/ProviderForm.vue` | 添加/编辑共用的表单页面 |

### 重构组件

| 组件 | 变更 |
|---|---|
| App.vue | 主路由：Config 页面切换为供应商列表 + 表单页面 |
| ConfigListMain.vue | 移除原有 mock 数据，改为供应商列表 |

### 新增 Composable

| Composable | 路径 | 职责 |
|---|---|---|
| useProviders.ts | `src/composables/useProviders.ts` | 供应商 CRUD、激活、测试连接 |

## 9. 架构分工

- **前端（Vue）**：UI 渲染、表单交互、校验、页面路由、状态管理
- **后端（Rust）**：文件 IO（读写 providers.json、读写 settings.json、测试连接）

## 10. 验收标准

1. 主界面正确显示所有供应商卡片，激活状态有视觉区分
2. 添加供应商：必填字段校验通过后保存成功
3. 编辑供应商：回填默认值，修改后保存成功
4. 复制供应商：创建副本并跳转编辑页面
5. 激活供应商：settings.json 中对应的 env 字段和 model 被正确更新
6. 删除供应商：确认后从列表中移除
7. 测试连接：正确发起 API 请求并返回结果
8. `~/.cc-desk/` 目录在首次启动时自动创建
9. 应用重启后，供应商列表和激活状态正确恢复
