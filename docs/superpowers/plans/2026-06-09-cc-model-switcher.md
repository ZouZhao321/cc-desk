# CC Model Switcher 实现计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**目标:** 构建 Tauri 2 桌面应用，可视化管理 Claude Code 模型配置（`~/.claude/settings.json`），支持预设切换、卡片式编辑、安全字段隔离。

**架构:** Rust 后端只读写 settings.json 中模型相关字段，采用原子写入+备份策略。Vue 3 前端使用 Naive UI 卡片展示/编辑配置，预设列表实现快速切换。预设通过 tauri-plugin-store 持久化。

**技术栈:** Tauri 2, Vue 3, TypeScript, Vite, Naive UI, UnoCSS, tauri-plugin-store, serde/serde_json (Rust)

---

## 文件结构

```
cc-workbench/
├── src-tauri/
│   ├── Cargo.toml                    # 修改: 添加 tauri-plugin-store, 移除 tauri-plugin-opener
│   ├── tauri.conf.json               # 修改: 窗口标题、尺寸
│   ├── capabilities/
│   │   └── default.json              # 修改: 添加 store 权限
│   └── src/
│       ├── lib.rs                    # 修改: 注册 store 插件, 替换 greet 为新命令
│       ├── main.rs                   # 不变
│       └── commands.rs              # 新建: settings.json 读写的 Tauri 命令
├── src/
│   ├── App.vue                       # 修改: 替换脚手架为应用布局
│   ├── main.ts                       # 修改: 注册 Naive UI + UnoCSS
│   ├── types/
│   │   └── index.ts                  # 新建: ModelConfig, Preset, PresetStore 类型
│   ├── utils/
│   │   └── mask.ts                   # 新建: token 脱敏工具
│   ├── composables/
│   │   ├── useSettings.ts            # 新建: settings.json 读写逻辑
│   │   └── usePresets.ts             # 新建: 预设管理逻辑
│   └── components/
│       ├── ConfigCard.vue            # 新建: 单个配置字段卡片（展示+编辑模式）
│       ├── PresetList.vue            # 新建: 预设列表（应用/删除）
│       └── PresetDialog.vue          # 新建: 新建预设弹窗
├── uno.config.ts                     # 新建: UnoCSS 配置
└── vite.config.ts                    # 修改: 添加 UnoCSS 插件
```

---

## 任务 1: 安装依赖 & 配置构建工具

**文件:**

- 修改: `package.json`（通过 pnpm）
- 修改: `vite.config.ts`
- 新建: `uno.config.ts`
- 修改: `src/main.ts`
- 修改: `src-tauri/Cargo.toml`
- 修改: `src-tauri/capabilities/default.json`
- 修改: `src-tauri/tauri.conf.json`
- 修改: `src-tauri/src/lib.rs`

- [ ] **步骤 1: 安装前端依赖**

```bash
cd C:/Users/ZouZhao/Desktop/cc-workbench
pnpm add naive-ui @tauri-apps/plugin-store
pnpm add -D unocss @unocss/preset-uno @unocss/preset-attributify
```

- [ ] **步骤 2: 确认 package.json 已更新**

读取 `package.json`，确认 `naive-ui`、`@tauri-apps/plugin-store`、`unocss`、`@unocss/preset-uno`、`@unocss/preset-attributify` 已出现在 dependencies/devDependencies 中。

- [ ] **步骤 3: 创建 UnoCSS 配置**

新建 `uno.config.ts`：

```ts
import { defineConfig, presetUno, presetAttributify } from 'unocss'

export default defineConfig({
	presets: [presetUno(), presetAttributify()]
})
```

- [ ] **步骤 4: 更新 vite.config.ts 添加 UnoCSS 插件**

用以下内容替换 `vite.config.ts`：

```ts
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import UnoCSS from 'unocss/vite'

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST

export default defineConfig(async () => ({
	plugins: [vue(), UnoCSS()],
	clearScreen: false,
	server: {
		port: 1420,
		strictPort: true,
		host: host || false,
		hmr: host
			? {
					protocol: 'ws',
					host,
					port: 1421
				}
			: undefined,
		watch: {
			ignored: ['**/src-tauri/**']
		}
	}
}))
```

- [ ] **步骤 5: 更新 src/main.ts 导入 UnoCSS**

用以下内容替换 `src/main.ts`：

```ts
import { createApp } from 'vue'
import App from './App.vue'
import 'virtual:uno.css'

createApp(App).mount('#app')
```

- [ ] **步骤 6: 更新 Cargo.toml — 添加 tauri-plugin-store，移除 tauri-plugin-opener**

用以下内容替换 `src-tauri/Cargo.toml` 的 `[dependencies]` 部分：

```toml
[dependencies]
tauri = { version = "2", features = [] }
tauri-plugin-store = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
dirs = "6"
```

- [ ] **步骤 7: 更新权限配置 — 添加 store 权限**

用以下内容替换 `src-tauri/capabilities/default.json`：

```json
{
	"$schema": "../gen/schemas/desktop-schema.json",
	"identifier": "default",
	"description": "Capability for the main window",
	"windows": ["main"],
	"permissions": ["core:default", "store:default"]
}
```

- [ ] **步骤 8: 更新 tauri.conf.json — 窗口标题和尺寸**

用以下内容替换 `src-tauri/tauri.conf.json`：

```json
{
	"$schema": "https://schema.tauri.app/config/2",
	"productName": "cc-model-switcher",
	"version": "0.1.0",
	"identifier": "com.zouzhao.cc-model-switcher",
	"build": {
		"beforeDevCommand": "pnpm dev",
		"devUrl": "http://localhost:1420",
		"beforeBuildCommand": "pnpm build",
		"frontendDist": "../dist"
	},
	"app": {
		"windows": [
			{
				"title": "CC Model Switcher",
				"width": 720,
				"height": 640,
				"resizable": true
			}
		],
		"security": {
			"csp": null
		}
	},
	"bundle": {
		"active": true,
		"targets": "all",
		"icon": ["icons/32x32.png", "icons/128x128.png", "icons/128x128@2x.png", "icons/icon.icns", "icons/icon.ico"]
	}
}
```

- [ ] **步骤 9: 更新 lib.rs — 注册 store 插件，移除 opener 和 greet**

用以下内容替换 `src-tauri/src/lib.rs`：

```rust
mod commands;

use commands::{get_settings_path, read_model_config, write_model_config};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            read_model_config,
            write_model_config,
            get_settings_path
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **步骤 10: 验证 Rust 编译**

```bash
cd C:/Users/ZouZhao/Desktop/cc-workbench/src-tauri && cargo check 2>&1
```

预期：编译报错（因为 `commands.rs` 还不存在），这是正常的 — 任务 2 会创建它。

- [ ] **步骤 11: 提交**

```bash
cd C:/Users/ZouZhao/Desktop/cc-workbench
git add package.json pnpm-lock.yaml vite.config.ts uno.config.ts src/main.ts src-tauri/Cargo.toml src-tauri/Cargo.lock src-tauri/capabilities/default.json src-tauri/tauri.conf.json src-tauri/src/lib.rs
git commit -m "chore: 安装依赖 (naive-ui, unocss, tauri-plugin-store) 并配置构建工具"
```

---

## 任务 2: Rust 后端 — 类型与命令

**文件:**

- 新建: `src-tauri/src/commands.rs`

- [ ] **步骤 1: 创建 commands.rs 完整实现**

新建 `src-tauri/src/commands.rs`：

```rust
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::command;

/// 从 ~/.claude/settings.json 提取的模型相关字段
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelConfig {
    pub auth_token: String,
    pub base_url: String,
    pub model: String,
    pub reasoning_model: String,
    pub haiku_id: String,
    pub haiku_name: String,
    pub sonnet_id: String,
    pub sonnet_name: String,
    pub opus_id: String,
    pub opus_name: String,
}

/// 返回 ~/.claude/settings.json 的路径
#[command]
pub fn get_settings_path() -> Result<String, String> {
    let home = dirs::home_dir().ok_or_else(|| "无法确定用户主目录".to_string())?;
    let path = home.join(".claude").join("settings.json");
    Ok(path.to_string_lossy().to_string())
}

/// 从 settings.json 读取模型相关字段
#[command]
pub fn read_model_config() -> Result<ModelConfig, String> {
    let path = settings_path()?;
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("读取 settings.json 失败: {e}"))?;
    let json: serde_json::Value =
        serde_json::from_str(&content).map_err(|e| format!("解析 settings.json 失败: {e}"))?;

    let env = json.get("env").cloned().unwrap_or_default();

    Ok(ModelConfig {
        auth_token: env_str(&env, "ANTHROPIC_AUTH_TOKEN"),
        base_url: env_str(&env, "ANTHROPIC_BASE_URL"),
        model: json_str(&json, "model"),
        reasoning_model: env_str(&env, "ANTHROPIC_REASONING_MODEL"),
        haiku_id: env_str(&env, "ANTHROPIC_DEFAULT_HAIKU_MODEL"),
        haiku_name: env_str(&env, "ANTHROPIC_DEFAULT_HAIKU_MODEL_NAME"),
        sonnet_id: env_str(&env, "ANTHROPIC_DEFAULT_SONNET_MODEL"),
        sonnet_name: env_str(&env, "ANTHROPIC_DEFAULT_SONNET_MODEL_NAME"),
        opus_id: env_str(&env, "ANTHROPIC_DEFAULT_OPUS_MODEL"),
        opus_name: env_str(&env, "ANTHROPIC_DEFAULT_OPUS_MODEL_NAME"),
    })
}

/// 将模型相关字段写入 settings.json（原子写入 + 备份）
#[command]
pub fn write_model_config(config: ModelConfig) -> Result<(), String> {
    let path = settings_path()?;

    // 读取完整 settings
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("读取 settings.json 失败: {e}"))?;
    let mut json: serde_json::Value =
        serde_json::from_str(&content).map_err(|e| format!("解析 settings.json 失败: {e}"))?;

    // 确保 env 对象存在
    if json.get("env").is_none() {
        json["env"] = serde_json::json!({});
    }
    let env = json["env"]
        .as_object_mut()
        .ok_or("env 不是对象类型")?;

    // 只写入模型相关字段
    env.insert("ANTHROPIC_AUTH_TOKEN".into(), serde_json::json!(config.auth_token));
    env.insert("ANTHROPIC_BASE_URL".into(), serde_json::json!(config.base_url));
    env.insert("ANTHROPIC_MODEL".into(), serde_json::json!(config.model));
    env.insert(
        "ANTHROPIC_REASONING_MODEL".into(),
        serde_json::json!(config.reasoning_model),
    );
    env.insert("ANTHROPIC_DEFAULT_HAIKU_MODEL".into(), serde_json::json!(config.haiku_id));
    env.insert(
        "ANTHROPIC_DEFAULT_HAIKU_MODEL_NAME".into(),
        serde_json::json!(config.haiku_name),
    );
    env.insert(
        "ANTHROPIC_DEFAULT_SONNET_MODEL".into(),
        serde_json::json!(config.sonnet_id),
    );
    env.insert(
        "ANTHROPIC_DEFAULT_SONNET_MODEL_NAME".into(),
        serde_json::json!(config.sonnet_name),
    );
    env.insert("ANTHROPIC_DEFAULT_OPUS_MODEL".into(), serde_json::json!(config.opus_id));
    env.insert(
        "ANTHROPIC_DEFAULT_OPUS_MODEL_NAME".into(),
        serde_json::json!(config.opus_name),
    );

    // 更新顶层 model 字段
    json["model"] = serde_json::json!(config.model);

    // 序列化
    let serialized =
        serde_json::to_string_pretty(&json).map_err(|e| format!("序列化失败: {e}"))?;

    // 备份: settings.json → settings.json.bak
    let bak_path = path.with_extension("json.bak");
    fs::copy(&path, &bak_path).map_err(|e| format!("创建备份失败: {e}"))?;

    // 原子写入: 先写 .tmp 再 rename
    let tmp_path = path.with_extension("json.tmp");
    fs::write(&tmp_path, &serialized).map_err(|e| format!("写入临时文件失败: {e}"))?;
    fs::rename(&tmp_path, &path).map_err(|e| format!("重命名临时文件失败: {e}"))?;

    Ok(())
}

fn settings_path() -> Result<PathBuf, String> {
    let home = dirs::home_dir().ok_or_else(|| "无法确定用户主目录".to_string())?;
    Ok(home.join(".claude").join("settings.json"))
}

fn env_str(env: &serde_json::Value, key: &str) -> String {
    env.get(key)
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string()
}

fn json_str(json: &serde_json::Value, key: &str) -> String {
    json.get(key)
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string()
}
```

- [ ] **步骤 2: 验证 Rust 编译**

```bash
cd C:/Users/ZouZhao/Desktop/cc-workbench/src-tauri && cargo check 2>&1
```

预期：`Finished`，无错误。

- [ ] **步骤 3: 运行 clippy**

```bash
cd C:/Users/ZouZhao/Desktop/cc-workbench/src-tauri && cargo clippy -- -D warnings 2>&1
```

预期：无警告。

- [ ] **步骤 4: 提交**

```bash
cd C:/Users/ZouZhao/Desktop/cc-workbench
git add src-tauri/src/commands.rs src-tauri/src/lib.rs
git commit -m "feat(rust): 添加模型配置读写命令，支持原子写入和备份"
```

---

## 任务 3: TypeScript 类型与工具

**文件:**

- 新建: `src/types/index.ts`
- 新建: `src/utils/mask.ts`

- [ ] **步骤 1: 创建类型定义**

新建 `src/types/index.ts`：

```ts
/** ~/.claude/settings.json 中的模型相关字段 */
export interface ModelConfig {
	auth_token: string
	base_url: string
	model: string
	reasoning_model: string
	haiku_id: string
	haiku_name: string
	sonnet_id: string
	sonnet_name: string
	opus_id: string
	opus_name: string
}

/** 已保存的预设 */
export interface Preset {
	id: string
	name: string
	config: ModelConfig
}

/** 预设存储数据（通过 tauri-plugin-store 持久化） */
export interface PresetStore {
	presets: Preset[]
	active_preset_id: string | null
}

/** 单个配置卡片的元数据 */
export interface ConfigField {
	key: keyof ModelConfig
	label: string
	icon: string
	sensitive: boolean
}
```

- [ ] **步骤 2: 创建 token 脱敏工具**

新建 `src/utils/mask.ts`：

```ts
/**
 * 脱敏显示敏感字符串。
 * "sk-abc123xyz" → "sk-a...xyz"
 * 短字符串（≤8 字符）全部用星号替代。
 */
export function maskToken(value: string): string {
	if (!value) return ''
	if (value.length <= 8) return '*'.repeat(value.length)
	const prefix = value.slice(0, 3)
	const suffix = value.slice(-3)
	return `${prefix}...${suffix}`
}
```

- [ ] **步骤 3: 提交**

```bash
cd C:/Users/ZouZhao/Desktop/cc-workbench
git add src/types/index.ts src/utils/mask.ts
git commit -m "feat(types): 添加 ModelConfig、Preset 类型定义和 token 脱敏工具"
```

---

## 任务 4: 组合式函数 — useSettings

**文件:**

- 新建: `src/composables/useSettings.ts`

- [ ] **步骤 1: 创建 useSettings 组合式函数**

新建 `src/composables/useSettings.ts`：

```ts
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { ModelConfig } from '../types'

const DEFAULT_CONFIG: ModelConfig = {
	auth_token: '',
	base_url: '',
	model: '',
	reasoning_model: '',
	haiku_id: '',
	haiku_name: '',
	sonnet_id: '',
	sonnet_name: '',
	opus_id: '',
	opus_name: ''
}

export function useSettings() {
	const config = ref<ModelConfig>({ ...DEFAULT_CONFIG })
	const dirty = ref(false)
	const loading = ref(false)
	const error = ref<string | null>(null)

	async function loadConfig() {
		loading.value = true
		error.value = null
		try {
			const result = await invoke<ModelConfig>('read_model_config')
			config.value = result
		} catch (e) {
			error.value = String(e)
		} finally {
			loading.value = false
		}
	}

	async function saveConfig() {
		loading.value = true
		error.value = null
		try {
			await invoke('write_model_config', { config: config.value })
			dirty.value = false
		} catch (e) {
			error.value = String(e)
		} finally {
			loading.value = false
		}
	}

	function updateField<K extends keyof ModelConfig>(key: K, value: ModelConfig[K]) {
		config.value[key] = value
		dirty.value = true
	}

	function applyConfig(newConfig: ModelConfig) {
		config.value = { ...newConfig }
		dirty.value = true
	}

	return { config, dirty, loading, error, loadConfig, saveConfig, updateField, applyConfig }
}
```

- [ ] **步骤 2: 提交**

```bash
cd C:/Users/ZouZhao/Desktop/cc-workbench
git add src/composables/useSettings.ts
git commit -m "feat(composable): 添加 useSettings 组合式函数用于读写模型配置"
```

---

## 任务 5: 组合式函数 — usePresets

**文件:**

- 新建: `src/composables/usePresets.ts`

- [ ] **步骤 1: 创建 usePresets 组合式函数**

新建 `src/composables/usePresets.ts`：

```ts
import { ref, onMounted } from 'vue'
import { load } from '@tauri-apps/plugin-store'
import type { ModelConfig, Preset, PresetStore } from '../types'

const STORE_FILE = 'presets.json'

export function usePresets() {
	const presets = ref<Preset[]>([])
	const activePresetId = ref<string | null>(null)
	const loading = ref(false)

	async function loadPresets() {
		loading.value = true
		try {
			const store = await load(STORE_FILE, { autoSave: false })
			const data = await store.get<PresetStore>('data')
			if (data) {
				presets.value = data.presets
				activePresetId.value = data.active_preset_id
			}
		} catch {
			// 存储文件不存在 — 从空状态开始
			presets.value = []
			activePresetId.value = null
		} finally {
			loading.value = false
		}
	}

	async function persistStore() {
		const store = await load(STORE_FILE, { autoSave: false })
		const data: PresetStore = {
			presets: presets.value,
			active_preset_id: activePresetId.value
		}
		await store.set('data', data)
		await store.save()
	}

	async function addPreset(name: string, config: ModelConfig) {
		const id = crypto.randomUUID()
		presets.value.push({ id, name, config })
		activePresetId.value = id
		await persistStore()
	}

	async function deletePreset(id: string) {
		presets.value = presets.value.filter(p => p.id !== id)
		if (activePresetId.value === id) {
			activePresetId.value = null
		}
		await persistStore()
	}

	async function applyPreset(id: string): Promise<ModelConfig | null> {
		const preset = presets.value.find(p => p.id === id)
		if (!preset) return null
		activePresetId.value = id
		await persistStore()
		return preset.config
	}

	onMounted(loadPresets)

	return { presets, activePresetId, loading, addPreset, deletePreset, applyPreset }
}
```

- [ ] **步骤 2: 提交**

```bash
cd C:/Users/ZouZhao/Desktop/cc-workbench
git add src/composables/usePresets.ts
git commit -m "feat(composable): 添加 usePresets 组合式函数用于预设管理"
```

---

## 任务 6: UI 组件 — ConfigCard

**文件:**

- 新建: `src/components/ConfigCard.vue`

- [ ] **步骤 1: 创建 ConfigCard 组件**

新建 `src/components/ConfigCard.vue`：

```vue
<script setup lang="ts">
import { ref } from 'vue'
import { NCard, NInput, NButton, NSpace } from 'naive-ui'
import { maskToken } from '../utils/mask'

const props = defineProps<{
	label: string
	icon: string
	modelValue: string
	sensitive: boolean
}>()

const emit = defineEmits<{
	'update:modelValue': [value: string]
}>()

const editing = ref(false)
const editValue = ref('')

function startEdit() {
	editValue.value = props.modelValue
	editing.value = true
}

function confirmEdit() {
	emit('update:modelValue', editValue.value)
	editing.value = false
}

function cancelEdit() {
	editing.value = false
}

function displayValue(): string {
	if (!props.modelValue) return '(未设置)'
	if (props.sensitive) return maskToken(props.modelValue)
	return props.modelValue
}
</script>

<template>
	<n-card size="small" hoverable class="w-72 cursor-pointer" @click="startEdit">
		<div class="flex items-center gap-2 mb-2">
			<span class="text-lg">{{ icon }}</span>
			<span class="font-bold text-sm">{{ label }}</span>
		</div>

		<div v-if="!editing" class="text-gray-600 dark:text-gray-300 text-sm truncate">
			{{ displayValue() }}
		</div>

		<div v-else @click.stop>
			<n-space vertical :size="8">
				<n-input
					v-model:value="editValue"
					size="small"
					:type="sensitive ? 'password' : 'text'"
					:placeholder="label"
					@keyup.enter="confirmEdit"
					@keyup.escape="cancelEdit"
				/>
				<n-space :size="4" justify="end">
					<n-button size="tiny" @click="cancelEdit">取消</n-button>
					<n-button size="tiny" type="primary" @click="confirmEdit">确认</n-button>
				</n-space>
			</n-space>
		</div>
	</n-card>
</template>
```

- [ ] **步骤 2: 提交**

```bash
cd C:/Users/ZouZhao/Desktop/cc-workbench
git add src/components/ConfigCard.vue
git commit -m "feat(ui): 添加 ConfigCard 组件，支持展示/编辑模式和 token 脱敏"
```

---

## 任务 7: UI 组件 — PresetList

**文件:**

- 新建: `src/components/PresetList.vue`

- [ ] **步骤 1: 创建 PresetList 组件**

新建 `src/components/PresetList.vue`：

```vue
<script setup lang="ts">
import { NCard, NButton, NSpace, NList, NListItem, NTag, NEmpty } from 'naive-ui'
import type { Preset } from '../types'

defineProps<{
	presets: Preset[]
	activePresetId: string | null
}>()

const emit = defineEmits<{
	apply: [id: string]
	delete: [id: string]
	newPreset: []
}>()
</script>

<template>
	<n-card title="预设管理" size="small">
		<template #header-extra>
			<n-button size="small" type="primary" @click="emit('newPreset')">+ 新建预设</n-button>
		</template>

		<n-empty v-if="presets.length === 0" description="暂无预设" class="py-4" />

		<n-list v-else bordered :show-divider="false">
			<n-list-item v-for="preset in presets" :key="preset.id">
				<n-space align="center" justify="space-between" class="w-full">
					<n-space align="center" :size="8">
						<span>{{ preset.name }}</span>
						<n-tag v-if="preset.id === activePresetId" type="success" size="small">当前</n-tag>
					</n-space>
					<n-space :size="4">
						<n-button size="tiny" @click="emit('apply', preset.id)">应用</n-button>
						<n-button size="tiny" type="error" @click="emit('delete', preset.id)">删除</n-button>
					</n-space>
				</n-space>
			</n-list-item>
		</n-list>
	</n-card>
</template>
```

- [ ] **步骤 2: 提交**

```bash
cd C:/Users/ZouZhao/Desktop/cc-workbench
git add src/components/PresetList.vue
git commit -m "feat(ui): 添加 PresetList 组件，支持应用/删除操作"
```

---

## 任务 8: UI 组件 — PresetDialog

**文件:**

- 新建: `src/components/PresetDialog.vue`

- [ ] **步骤 1: 创建 PresetDialog 组件**

新建 `src/components/PresetDialog.vue`：

```vue
<script setup lang="ts">
import { ref, watch } from 'vue'
import { NModal, NInput, NSpace, NButton } from 'naive-ui'

const props = defineProps<{
	show: boolean
}>()

const emit = defineEmits<{
	'update:show': [value: boolean]
	save: [name: string]
}>()

const presetName = ref('')

watch(
	() => props.show,
	val => {
		if (val) presetName.value = ''
	}
)

function handleSave() {
	if (!presetName.value.trim()) return
	emit('save', presetName.value.trim())
	emit('update:show', false)
}
</script>

<template>
	<n-modal :show="show" preset="dialog" title="新建预设" @update:show="emit('update:show', $event)">
		<n-space vertical :size="12">
			<n-input v-model:value="presetName" placeholder="输入预设名称" @keyup.enter="handleSave" />
			<n-space justify="end">
				<n-button @click="emit('update:show', false)">取消</n-button>
				<n-button type="primary" :disabled="!presetName.trim()" @click="handleSave">保存</n-button>
			</n-space>
		</n-space>
	</n-modal>
</template>
```

- [ ] **步骤 2: 提交**

```bash
cd C:/Users/ZouZhao/Desktop/cc-workbench
git add src/components/PresetDialog.vue
git commit -m "feat(ui): 添加 PresetDialog 新建预设弹窗组件"
```

---

## 任务 9: 主应用布局

**文件:**

- 修改: `src/App.vue`
- 修改: `index.html`

- [ ] **步骤 1: 替换 App.vue 为主应用布局**

用以下内容替换 `src/App.vue`：

```vue
<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { NConfigProvider, NSpace, NButton, NH2, NSpin, NAlert, NMessageProvider } from 'naive-ui'
import ConfigCard from './components/ConfigCard.vue'
import PresetList from './components/PresetList.vue'
import PresetDialog from './components/PresetDialog.vue'
import { useSettings } from './composables/useSettings'
import { usePresets } from './composables/usePresets'
import type { ConfigField } from './types'

const { config, dirty, loading, error, loadConfig, saveConfig, updateField, applyConfig } = useSettings()
const { presets, activePresetId, addPreset, deletePreset, applyPreset } = usePresets()

const showDialog = ref(false)

const configFields: ConfigField[] = [
	{ key: 'auth_token', label: 'API Token', icon: '🔑', sensitive: true },
	{ key: 'base_url', label: 'Base URL', icon: '🌐', sensitive: false },
	{ key: 'model', label: 'Model', icon: '🤖', sensitive: false },
	{ key: 'reasoning_model', label: 'Reasoning Model', icon: '🧠', sensitive: false },
	{ key: 'haiku_id', label: 'Haiku ID', icon: '🟢', sensitive: false },
	{ key: 'haiku_name', label: 'Haiku Name', icon: '🟢', sensitive: false },
	{ key: 'sonnet_id', label: 'Sonnet ID', icon: '🔵', sensitive: false },
	{ key: 'sonnet_name', label: 'Sonnet Name', icon: '🔵', sensitive: false },
	{ key: 'opus_id', label: 'Opus ID', icon: '🟣', sensitive: false },
	{ key: 'opus_name', label: 'Opus Name', icon: '🟣', sensitive: false }
]

async function handleApplyPreset(id: string) {
	const presetConfig = await applyPreset(id)
	if (presetConfig) {
		applyConfig(presetConfig)
		await saveConfig()
	}
}

async function handleDeletePreset(id: string) {
	await deletePreset(id)
}

async function handleSavePreset(name: string) {
	await addPreset(name, config.value)
}

async function handleSave() {
	await saveConfig()
}

onMounted(loadConfig)
</script>

<template>
	<n-config-provider>
		<n-message-provider>
			<div class="p-6 max-w-3xl mx-auto">
				<n-space align="center" justify="space-between" class="mb-4">
					<n-h2 class="!mb-0">CC Model Switcher</n-h2>
					<n-button type="primary" :disabled="!dirty || loading" :loading="loading" @click="handleSave">
						保存
					</n-button>
				</n-space>

				<n-alert v-if="error" type="error" class="mb-4" closable>
					{{ error }}
				</n-alert>

				<n-space v-if="loading && !config.auth_token" justify="center" class="py-12">
					<n-spin size="large" />
				</n-space>

				<template v-else>
					<div class="mb-6">
						<div class="text-sm text-gray-500 mb-3">当前配置</div>
						<div class="flex flex-wrap gap-3">
							<config-card
								v-for="field in configFields"
								:key="field.key"
								:label="field.label"
								:icon="field.icon"
								:model-value="config[field.key]"
								:sensitive="field.sensitive"
								@update:model-value="updateField(field.key, $event)"
							/>
						</div>
					</div>

					<preset-list
						:presets="presets"
						:active-preset-id="activePresetId"
						@apply="handleApplyPreset"
						@delete="handleDeletePreset"
						@new-preset="showDialog = true"
					/>
				</template>

				<preset-dialog v-model:show="showDialog" @save="handleSavePreset" />
			</div>
		</n-message-provider>
	</n-config-provider>
</template>
```

- [ ] **步骤 2: 更新 index.html 标题**

将 `index.html` 中的 `<title>` 替换为：

```html
<title>CC Model Switcher</title>
```

- [ ] **步骤 3: 验证前端编译**

```bash
cd C:/Users/ZouZhao/Desktop/cc-workbench && pnpm build 2>&1
```

预期：`vue-tsc --noEmit` 通过，`vite build` 成功。

- [ ] **步骤 4: 提交**

```bash
cd C:/Users/ZouZhao/Desktop/cc-workbench
git add src/App.vue index.html
git commit -m "feat(ui): 主应用布局，包含配置卡片、预设列表和保存按钮"
```

---

## 任务 10: 端到端验证

**文件：** 无（仅验证）

- [ ] **步骤 1: 运行前端 lint**

```bash
cd C:/Users/ZouZhao/Desktop/cc-workbench && pnpm lint 2>&1
```

预期：无错误。

- [ ] **步骤 2: 运行 Rust clippy**

```bash
cd C:/Users/ZouZhao/Desktop/cc-workbench/src-tauri && cargo clippy -- -D warnings 2>&1
```

预期：无警告。

- [ ] **步骤 3: 运行完整构建（前端 + Rust）**

```bash
cd C:/Users/ZouZhao/Desktop/cc-workbench && pnpm tauri build 2>&1 | tail -30
```

预期：构建成功完成。

- [ ] **步骤 4: 手动冒烟测试**

运行 `pnpm tauri dev` 启动窗口，验证：

1. 配置卡片显示 `~/.claude/settings.json` 中的值
2. 点击卡片进入编辑模式，回车/确认后更新值
3. 点击"保存"写入 `settings.json`（验证文件已变更）
4. 创建预设保存成功；应用预设后卡片更新并写入文件
5. 删除预设后从列表中移除
6. Token 字段显示脱敏值；编辑时显示完整值

- [ ] **步骤 5: 如有修复则最终提交**

```bash
cd C:/Users/ZouZhao/Desktop/cc-workbench
git add -A
git commit -m "fix: 修复端到端验证中发现的问题"
```

仅在冒烟测试发现问题并修复时执行。
