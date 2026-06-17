# LLM 供应商配置功能实现计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 实现供应商管理功能，支持多供应商配置的增删改查、一键切换激活。

**Architecture:** Rust 后端负责文件 IO（providers.json + settings.json），前端 Vue 负责 UI 渲染和交互。数据流：前端 → invoke Rust 命令 → 读写文件 → 返回结果。

**Tech Stack:** Rust (Tauri 2), Vue 3 + TypeScript, Naive UI, UnoCSS, reqwest (HTTP client)

---

## 文件结构

| 文件 | 操作 | 职责 |
|---|---|---|
| `src-tauri/src/commands.rs` | 修改 | 新增 Provider 结构体、list/save/activate/test 命令 |
| `src-tauri/src/lib.rs` | 修改 | 注册新命令 |
| `src-tauri/Cargo.toml` | 修改 | 添加 reqwest 依赖 |
| `src/types/index.ts` | 修改 | 新增 Provider 接口 |
| `src/composables/useProviders.ts` | 新建 | 供应商 CRUD + 激活 + 测试连接 |
| `src/components/ProviderCard.vue` | 新建 | 供应商卡片组件 |
| `src/components/ProviderForm.vue` | 新建 | 添加/编辑表单组件 |
| `src/App.vue` | 修改 | 重构路由，集成供应商管理页面 |
| `src/components/ConfigListMain.vue` | 修改 | 改为供应商列表 |

---

## Task 1: Rust — 添加 Provider 结构体和 providers.json IO 命令

**Files:**
- Modify: `src-tauri/src/commands.rs:1-20` (在 ModelConfig 后添加 Provider)
- Modify: `src-tauri/src/lib.rs:3-6` (导入新命令)
- Modify: `src-tauri/src/lib.rs:12-21` (注册新命令)

- [ ] **Step 1: 在 commands.rs 中添加 Provider 结构体**

在 `ModelConfig` 结构体之后（第 19 行后）添加：

```rust
/// 供应商配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Provider {
    pub id: String,
    pub name: String,
    pub notes: Option<String>,
    pub website: Option<String>,
    pub api_key: String,
    pub base_url: String,
    pub main_model: String,
    pub opus_model: String,
    pub sonnet_model: String,
    pub haiku_model: String,
    pub sub_agent_model: String,
    pub reasoning_level: String,
    pub is_active: bool,
}

/// 供应商存储数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProviderStore {
    pub providers: Vec<Provider>,
}
```

- [ ] **Step 2: 添加 providers.json 路径函数**

在 `settings_path()` 函数之后（第 127 行后）添加：

```rust
/// 返回 ~/.cc-desk/providers.json 的路径（自动创建目录）
fn providers_path() -> Result<PathBuf, String> {
    let home = dirs::home_dir().ok_or_else(|| "无法确定用户主目录".to_string())?;
    let dir = home.join(".cc-desk");
    fs::create_dir_all(&dir).map_err(|e| format!("创建 .cc-desk 目录失败: {e}"))?;
    Ok(dir.join("providers.json"))
}
```

- [ ] **Step 3: 添加 list_providers 命令**

在 `providers_path()` 函数之后添加：

```rust
/// 读取所有供应商配置
#[command]
pub fn list_providers() -> Result<Vec<Provider>, String> {
    let path = providers_path()?;
    if !path.exists() {
        return Ok(Vec::new());
    }
    let content = fs::read_to_string(&path).map_err(|e| format!("读取 providers.json 失败: {e}"))?;
    let store: ProviderStore =
        serde_json::from_str(&content).map_err(|e| format!("解析 providers.json 失败: {e}"))?;
    Ok(store.providers)
}
```

- [ ] **Step 4: 添加 save_providers 命令**

在 `list_providers` 命令之后添加：

```rust
/// 保存所有供应商配置（原子写入）
#[command]
pub fn save_providers(providers: Vec<Provider>) -> Result<(), String> {
    let path = providers_path()?;
    let store = ProviderStore { providers };
    let serialized = serde_json::to_string_pretty(&store).map_err(|e| format!("序列化失败: {e}"))?;

    let tmp_path = path.with_extension("json.tmp");
    fs::write(&tmp_path, &serialized).map_err(|e| format!("写入临时文件失败: {e}"))?;
    fs::rename(&tmp_path, &path).map_err(|e| format!("重命名临时文件失败: {e}"))?;

    Ok(())
}
```

- [ ] **Step 5: 在 lib.rs 中导入并注册新命令**

修改 `src-tauri/src/lib.rs`：

```rust
mod commands;

use commands::{
    get_session_last_message, get_settings_path, list_providers, list_sessions, load_annotations,
    read_model_config, read_session, save_annotation, save_providers, write_model_config,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            read_model_config,
            write_model_config,
            get_settings_path,
            list_providers,
            save_providers,
            list_sessions,
            read_session,
            load_annotations,
            save_annotation,
            get_session_last_message
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **Step 6: 编译验证**

Run: `cd src-tauri && cargo check`
Expected: 编译成功，无错误

- [ ] **Step 7: Commit**

```bash
git add src-tauri/src/commands.rs src-tauri/src/lib.rs
git commit -m "feat(rust): 添加 Provider 结构体和 providers.json IO 命令"
```

---

## Task 2: Rust — 添加 activate_provider 命令

**Files:**
- Modify: `src-tauri/src/commands.rs` (在 save_providers 命令之后添加)
- Modify: `src-tauri/src/lib.rs` (导入并注册)

- [ ] **Step 1: 添加 activate_provider 命令**

在 `save_providers` 命令之后添加：

```rust
/// 激活供应商：将配置写入 settings.json
#[command]
pub fn activate_provider(provider: Provider) -> Result<(), String> {
    let path = settings_path()?;

    // 读取完整 settings
    let content = fs::read_to_string(&path).map_err(|e| format!("读取 settings.json 失败: {e}"))?;
    let mut json: serde_json::Value =
        serde_json::from_str(&content).map_err(|e| format!("解析 settings.json 失败: {e}"))?;

    // 确保 env 对象存在
    if json.get("env").is_none() {
        json["env"] = serde_json::json!({});
    }
    let env = json["env"].as_object_mut().ok_or("env 不是对象类型")?;

    // 写入模型相关字段
    env.insert("ANTHROPIC_AUTH_TOKEN".into(), serde_json::json!(provider.api_key));
    env.insert("ANTHROPIC_BASE_URL".into(), serde_json::json!(provider.base_url));

    // ANTHROPIC_MODEL 根据 main_model 角色派生
    let derived_model_id = match provider.main_model.as_str() {
        "haiku" => &provider.haiku_model,
        "sonnet" => &provider.sonnet_model,
        "opus" => &provider.opus_model,
        _ => &provider.main_model,
    };
    env.insert("ANTHROPIC_MODEL".into(), serde_json::json!(derived_model_id));

    env.insert("ANTHROPIC_REASONING_MODEL".into(), serde_json::json!(provider.sub_agent_model));
    env.insert("ANTHROPIC_DEFAULT_HAIKU_MODEL".into(), serde_json::json!(provider.haiku_model));
    env.insert("ANTHROPIC_DEFAULT_SONNET_MODEL".into(), serde_json::json!(provider.sonnet_model));
    env.insert("ANTHROPIC_DEFAULT_OPUS_MODEL".into(), serde_json::json!(provider.opus_model));
    env.insert("CLAUDE_CODE_EFFORT_LEVEL".into(), serde_json::json!(provider.reasoning_level));

    // 删除 _MODEL_NAME 系列字段
    env.remove("ANTHROPIC_DEFAULT_HAIKU_MODEL_NAME");
    env.remove("ANTHROPIC_DEFAULT_SONNET_MODEL_NAME");
    env.remove("ANTHROPIC_DEFAULT_OPUS_MODEL_NAME");

    // 更新顶层 model 字段
    json["model"] = serde_json::json!(provider.main_model);

    // 序列化
    let serialized = serde_json::to_string_pretty(&json).map_err(|e| format!("序列化失败: {e}"))?;

    // 备份: settings.json -> settings.json.bak
    let bak_path = path.with_extension("json.bak");
    fs::copy(&path, &bak_path).map_err(|e| format!("创建备份失败: {e}"))?;

    // 原子写入: 先写 .tmp 再 rename
    let tmp_path = path.with_extension("json.tmp");
    fs::write(&tmp_path, &serialized).map_err(|e| format!("写入临时文件失败: {e}"))?;
    fs::rename(&tmp_path, &path).map_err(|e| format!("重命名临时文件失败: {e}"))?;

    Ok(())
}
```

- [ ] **Step 2: 在 lib.rs 中导入并注册**

修改 `src-tauri/src/lib.rs` 的 use 语句和 generate_handler：

```rust
use commands::{
    activate_provider, get_session_last_message, get_settings_path, list_providers,
    list_sessions, load_annotations, read_model_config, read_session, save_annotation,
    save_providers, write_model_config,
};

// ... invoke_handler 中添加 activate_provider
```

- [ ] **Step 3: 编译验证**

Run: `cd src-tauri && cargo check`
Expected: 编译成功

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/commands.rs src-tauri/src/lib.rs
git commit -m "feat(rust): 添加 activate_provider 命令，激活时写入 settings.json"
```

---

## Task 3: Rust — 添加 test_connection 命令

**Files:**
- Modify: `src-tauri/Cargo.toml` (添加 reqwest 依赖)
- Modify: `src-tauri/src/commands.rs` (添加 test_connection 命令)
- Modify: `src-tauri/src/lib.rs` (导入并注册)

- [ ] **Step 1: 添加 reqwest 依赖**

修改 `src-tauri/Cargo.toml` 的 `[dependencies]` 部分：

```toml
[dependencies]
tauri = { version = "2", features = [] }
tauri-plugin-store = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
dirs = "6"
reqwest = { version = "0.12", features = ["json"] }
tokio = { version = "1", features = ["full"] }
```

- [ ] **Step 2: 添加 test_connection 命令**

在 `activate_provider` 命令之后添加：

```rust
/// 测试 API 连通性
#[command]
pub async fn test_connection(api_key: String, base_url: String) -> Result<String, String> {
    let url = format!("{}/v1/models", base_url.trim_end_matches('/'));

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await
        .map_err(|e| format!("连接失败: {e}"))?;

    if response.status().is_success() {
        Ok("连接成功".to_string())
    } else {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        Err(format!("连接失败 ({}): {}", status, body))
    }
}
```

- [ ] **Step 3: 在 lib.rs 中导入并注册**

修改 `src-tauri/src/lib.rs`：

```rust
use commands::{
    activate_provider, get_session_last_message, get_settings_path, list_providers,
    list_sessions, load_annotations, read_model_config, read_session, save_annotation,
    save_providers, test_connection, write_model_config,
};

// ... invoke_handler 中添加 test_connection
```

- [ ] **Step 4: 编译验证**

Run: `cd src-tauri && cargo check`
Expected: 编译成功

- [ ] **Step 5: Commit**

```bash
git add src-tauri/Cargo.toml src-tauri/src/commands.rs src-tauri/src/lib.rs
git commit -m "feat(rust): 添加 test_connection 命令，支持 API 连通性测试"
```

---

## Task 4: Frontend — 添加 Provider 类型定义

**Files:**
- Modify: `src/types/index.ts` (在文件末尾添加)

- [ ] **Step 1: 添加 Provider 接口**

在 `src/types/index.ts` 文件末尾添加：

```typescript
/** 供应商配置 */
export interface Provider {
	id: string
	name: string
	notes?: string
	website?: string
	api_key: string
	base_url: string
	main_model: string
	opus_model: string
	sonnet_model: string
	haiku_model: string
	sub_agent_model: string
	reasoning_level: string
	is_active: boolean
}

/** 供应商存储数据 */
export interface ProviderStore {
	providers: Provider[]
}
```

- [ ] **Step 2: Commit**

```bash
git add src/types/index.ts
git commit -m "feat(types): 添加 Provider 类型定义"
```

---

## Task 5: Frontend — 创建 useProviders composable

**Files:**
- Create: `src/composables/useProviders.ts`

- [ ] **Step 1: 创建 useProviders composable**

```typescript
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { Provider } from '../types'

export function useProviders() {
	const providers = ref<Provider[]>([])
	const loading = ref(false)
	const error = ref<string | null>(null)

	async function loadProviders() {
		loading.value = true
		error.value = null
		try {
			const result = await invoke<Provider[]>('list_providers')
			providers.value = result
		} catch (e) {
			error.value = String(e)
		} finally {
			loading.value = false
		}
	}

	async function saveProviders() {
		loading.value = true
		error.value = null
		try {
			await invoke('save_providers', { providers: providers.value })
		} catch (e) {
			error.value = String(e)
		} finally {
			loading.value = false
		}
	}

	async function addProvider(provider: Omit<Provider, 'id' | 'is_active'>) {
		const newProvider: Provider = {
			...provider,
			id: crypto.randomUUID(),
			is_active: false
		}
		providers.value.push(newProvider)
		await saveProviders()
		return newProvider
	}

	async function updateProvider(updated: Provider) {
		const index = providers.value.findIndex(p => p.id === updated.id)
		if (index !== -1) {
			providers.value[index] = updated
			await saveProviders()
		}
	}

	async function deleteProvider(id: string) {
		providers.value = providers.value.filter(p => p.id !== id)
		await saveProviders()
	}

	async function duplicateProvider(id: string) {
		const source = providers.value.find(p => p.id === id)
		if (!source) return null
		const duplicate: Provider = {
			...source,
			id: crypto.randomUUID(),
			name: `${source.name}(副本)`,
			is_active: false
		}
		providers.value.push(duplicate)
		await saveProviders()
		return duplicate
	}

	async function activateProvider(provider: Provider) {
		providers.value.forEach(p => {
			p.is_active = p.id === provider.id
		})
		await invoke('activate_provider', { provider })
		await saveProviders()
	}

	async function testConnection(api_key: string, base_url: string) {
		try {
			const result = await invoke<string>('test_connection', { apiKey: api_key, baseUrl: base_url })
			return { success: true, message: result }
		} catch (e) {
			return { success: false, message: String(e) }
		}
	}

	return {
		providers,
		loading,
		error,
		loadProviders,
		saveProviders,
		addProvider,
		updateProvider,
		deleteProvider,
		duplicateProvider,
		activateProvider,
		testConnection
	}
}
```

- [ ] **Step 2: Commit**

```bash
git add src/composables/useProviders.ts
git commit -m "feat(composables): 添加 useProviders composable"
```

---

## Task 6: Frontend — 创建 ProviderCard 组件

**Files:**
- Create: `src/components/ProviderCard.vue`

- [ ] **Step 1: 创建 ProviderCard.vue**

```vue
<script setup lang="ts">
import type { Provider } from '../types'

defineProps<{
	provider: Provider
}>()

defineEmits<{
	activate: [provider: Provider]
	edit: [provider: Provider]
	duplicate: [id: string]
	delete: [id: string]
}>()
</script>

<template>
	<div
		class="flex items-center gap-16px p-16px pr-20px bg-white rounded-12px border cursor-pointer transition-all shadow-[0_2px_8px_rgba(0,0,0,0.03)] hover:border-gray-300 hover:shadow-[0_4px_12px_rgba(0,0,0,0.06)]"
		:class="provider.is_active ? '!border-[#3B82F6] !border-l-[3px]' : 'border-gray-200'"
	>
		<!-- 拖拽手柄 -->
		<div class="shrink-0 cursor-grab">
			<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="#D1D5DB" stroke-width="2">
				<circle cx="9" cy="6" r="1" /><circle cx="15" cy="6" r="1" />
				<circle cx="9" cy="12" r="1" /><circle cx="15" cy="12" r="1" />
				<circle cx="9" cy="18" r="1" /><circle cx="15" cy="18" r="1" />
			</svg>
		</div>

		<!-- Logo -->
		<div
			class="flex items-center justify-center w-40px h-40px rounded-full shrink-0"
			:class="provider.is_active ? 'bg-blue-50' : 'bg-gray-100'"
		>
			<span class="text-14px font-600" :class="provider.is_active ? 'text-blue-500' : 'text-gray-500'">
				{{ provider.name.charAt(0).toUpperCase() }}
			</span>
		</div>

		<!-- 信息 -->
		<div class="flex-1 flex flex-col gap-2px min-w-0">
			<span class="text-14px font-700 text-gray-900">{{ provider.name }}</span>
			<a
				v-if="provider.website"
				:href="provider.website"
				target="_blank"
				class="text-12px text-blue-500 no-underline truncate hover:underline"
				@click.stop
			>
				{{ provider.website }}
			</a>
		</div>

		<!-- 操作按钮 -->
		<div class="shrink-0 flex items-center gap-8px">
			<!-- 激活状态 -->
			<span
				v-if="provider.is_active"
				class="flex items-center gap-4px text-12px text-green-500 bg-green-50 px-8px py-4px rounded-4px"
			>
				<svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<polyline points="20 6 9 17 4 12" />
				</svg>
				使用中
			</span>
			<!-- 激活按钮 -->
			<button
				v-else
				class="flex items-center gap-4px text-12px text-gray-600 bg-gray-100 px-8px py-4px rounded-4px border-none cursor-pointer hover:bg-gray-200 transition-colors"
				@click.stop="$emit('activate', provider)"
			>
				<svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<path d="M22 11.08V12a10 10 0 1 1-5.93-9.14" />
					<polyline points="22 4 12 14.01 9 11.01" />
				</svg>
				激活
			</button>

			<!-- 复制 -->
			<button
				class="flex items-center justify-center w-28px h-28px rounded-6px border-none bg-transparent cursor-pointer hover:bg-gray-100 transition-colors"
				@click.stop="$emit('duplicate', provider.id)"
			>
				<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#6B7280" stroke-width="2">
					<rect x="9" y="9" width="13" height="13" rx="2" ry="2" />
					<path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" />
				</svg>
			</button>

			<!-- 编辑 -->
			<button
				class="flex items-center justify-center w-28px h-28px rounded-6px border-none bg-transparent cursor-pointer hover:bg-gray-100 transition-colors"
				@click.stop="$emit('edit', provider)"
			>
				<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#6B7280" stroke-width="2">
					<path d="M17 3a2.828 2.828 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5L17 3z" />
				</svg>
			</button>

			<!-- 删除 -->
			<button
				class="flex items-center justify-center w-28px h-28px rounded-6px border-none bg-transparent cursor-pointer hover:bg-red-50 transition-colors"
				@click.stop="$emit('delete', provider.id)"
			>
				<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#6B7280" stroke-width="2">
					<polyline points="3 6 5 6 21 6" />
					<path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" />
				</svg>
			</button>
		</div>
	</div>
</template>
```

- [ ] **Step 2: Commit**

```bash
git add src/components/ProviderCard.vue
git commit -m "feat(components): 添加 ProviderCard 供应商卡片组件"
```

---

## Task 7: Frontend — 创建 ProviderForm 组件

**Files:**
- Create: `src/components/ProviderForm.vue`

- [ ] **Step 1: 创建 ProviderForm.vue**

```vue
<script setup lang="ts">
import { ref, watch } from 'vue'
import { NInput, NSelect, NButton, NAlert } from 'naive-ui'
import type { Provider } from '../types'

const props = defineProps<{
	provider?: Provider | null
}>()

const emit = defineEmits<{
	save: [provider: Omit<Provider, 'id' | 'is_active'>]
	cancel: []
	test: [api_key: string, base_url: string]
}>()

const form = ref({
	name: '',
	notes: '',
	website: '',
	api_key: '',
	base_url: '',
	main_model: 'sonnet',
	opus_model: '',
	sonnet_model: '',
	haiku_model: '',
	sub_agent_model: 'haiku',
	reasoning_level: 'max'
})

const testing = ref(false)
const testResult = ref<{ success: boolean; message: string } | null>(null)

const modelOptions = [
	{ label: 'Haiku', value: 'haiku' },
	{ label: 'Sonnet', value: 'sonnet' },
	{ label: 'Opus', value: 'opus' }
]

const reasoningOptions = [
	{ label: 'Low', value: 'low' },
	{ label: 'Medium', value: 'medium' },
	{ label: 'High', value: 'high' },
	{ label: 'Max', value: 'max' },
	{ label: 'XHigh', value: 'xhigh' }
]

watch(
	() => props.provider,
	(p) => {
		if (p) {
			form.value = {
				name: p.name,
				notes: p.notes || '',
				website: p.website || '',
				api_key: p.api_key,
				base_url: p.base_url,
				main_model: p.main_model,
				opus_model: p.opus_model,
				sonnet_model: p.sonnet_model,
				haiku_model: p.haiku_model,
				sub_agent_model: p.sub_agent_model,
				reasoning_level: p.reasoning_level
			}
		}
	},
	{ immediate: true }
)

function handleSave() {
	if (!form.value.name || !form.value.api_key || !form.value.base_url) return
	emit('save', { ...form.value })
}

async function handleTest() {
	if (!form.value.api_key || !form.value.base_url) return
	testing.value = true
	testResult.value = null
	emit('test', form.value.api_key, form.value.base_url)
	testing.value = false
}
</script>

<template>
	<div class="flex flex-col w-full h-full bg-white font-sans">
		<!-- 头部 -->
		<header class="flex items-center justify-between h-56px px-24px bg-white border-b border-gray-100 shrink-0">
			<div class="flex items-center gap-12px">
				<button
					class="flex items-center justify-center w-32px h-32px border-none bg-gray-100 rounded-6px cursor-pointer hover:bg-gray-200 transition-colors"
					@click="emit('cancel')"
				>
					<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="#666666" stroke-width="2">
						<path d="M19 12H5M12 19l-7-7 7-7" />
					</svg>
				</button>
				<span class="text-20px font-700 text-gray-900">
					{{ provider ? '编辑供应商' : '添加供应商' }}
				</span>
			</div>
			<button
				class="text-14px text-gray-500 bg-transparent border-none cursor-pointer hover:text-gray-700"
				@click="emit('cancel')"
			>
				取消
			</button>
		</header>

		<!-- 表单内容 -->
		<div class="flex-1 overflow-y-auto px-32px py-24px">
			<div class="max-w-640px mx-auto flex flex-col gap-24px">
				<!-- 基本信息 -->
				<section class="flex flex-col gap-16px">
					<h3 class="text-16px font-600 text-gray-900 m-0">基本信息</h3>

					<div class="flex flex-col gap-6px">
						<label class="text-13px text-gray-600">供应商名称</label>
						<n-input v-model:value="form.name" placeholder="例如：DeepSeek" />
					</div>

					<div class="flex flex-col gap-6px">
						<label class="text-13px text-gray-600">备注</label>
						<n-input v-model:value="form.notes" placeholder="可选备注信息" />
					</div>

					<div class="flex flex-col gap-6px">
						<label class="text-13px text-gray-600">官网链接</label>
						<n-input v-model:value="form.website" placeholder="https://platform.example.com" />
					</div>
				</section>

				<!-- API 配置 -->
				<section class="flex flex-col gap-16px">
					<h3 class="text-16px font-600 text-gray-900 m-0">API 配置</h3>

					<div class="flex flex-col gap-6px">
						<label class="text-13px text-gray-600">API Key</label>
						<n-input v-model:value="form.api_key" type="password" show-password-on="click" placeholder="sk-xxxxxxxxxxxxxxxx" />
					</div>

					<div class="flex flex-col gap-6px">
						<label class="text-13px text-gray-600">请求地址</label>
						<n-input v-model:value="form.base_url" placeholder="https://api.example.com/anthropic" />
					</div>
				</section>

				<!-- 模型配置 -->
				<section class="flex flex-col gap-16px">
					<div>
						<h3 class="text-16px font-600 text-gray-900 m-0">模型配置</h3>
						<p class="text-13px text-gray-500 m-0 mt-4px">配置不同场景使用的模型</p>
					</div>

					<div class="grid grid-cols-2 gap-16px">
						<div class="flex flex-col gap-6px">
							<label class="text-13px text-gray-600">主模型</label>
							<n-select v-model:value="form.main_model" :options="modelOptions" />
						</div>
						<div class="flex flex-col gap-6px">
							<label class="text-13px text-gray-600">Opus 模型</label>
							<n-input v-model:value="form.opus_model" placeholder="模型 ID" />
						</div>
						<div class="flex flex-col gap-6px">
							<label class="text-13px text-gray-600">Sonnet 模型</label>
							<n-input v-model:value="form.sonnet_model" placeholder="模型 ID" />
						</div>
						<div class="flex flex-col gap-6px">
							<label class="text-13px text-gray-600">Haiku 模型</label>
							<n-input v-model:value="form.haiku_model" placeholder="模型 ID" />
						</div>
						<div class="flex flex-col gap-6px">
							<label class="text-13px text-gray-600">子代理模型</label>
							<n-select v-model:value="form.sub_agent_model" :options="modelOptions" />
						</div>
						<div class="flex flex-col gap-6px">
							<label class="text-13px text-gray-600">推理强度</label>
							<n-select v-model:value="form.reasoning_level" :options="reasoningOptions" />
						</div>
					</div>
				</section>

				<!-- 测试结果提示 -->
				<n-alert v-if="testResult" :type="testResult.success ? 'success' : 'error'" class="mt-8px">
					{{ testResult.message }}
				</n-alert>

				<!-- 操作栏 -->
				<div class="flex items-center justify-end gap-12px py-16px border-t border-gray-100">
					<n-button :loading="testing" @click="handleTest">
						<template #icon>
							<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
								<path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71" />
								<path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71" />
							</svg>
						</template>
						测试连接
					</n-button>
					<n-button type="primary" @click="handleSave">
						<template #icon>
							<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
								<polyline points="20 6 9 17 4 12" />
							</svg>
						</template>
						保存配置
					</n-button>
				</div>
			</div>
		</div>
	</div>
</template>
```

- [ ] **Step 2: Commit**

```bash
git add src/components/ProviderForm.vue
git commit -m "feat(components): 添加 ProviderForm 供应商表单组件"
```

---

## Task 8: Frontend — 重构 App.vue 和 ConfigListMain.vue

**Files:**
- Modify: `src/App.vue` (重构路由和布局)
- Modify: `src/components/ConfigListMain.vue` (改为供应商列表)

- [ ] **Step 1: 重构 App.vue**

替换 `src/App.vue` 的全部内容：

```vue
<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { NConfigProvider, NMessageProvider, useMessage } from 'naive-ui'
import ConfigListMain from './components/ConfigListMain.vue'
import ProviderForm from './components/ProviderForm.vue'
import SessionHistory from './components/SessionHistory.vue'
import { useProviders } from './composables/useProviders'
import type { Provider } from './types'

const message = useMessage()
const { providers, loading, loadProviders, addProvider, updateProvider, deleteProvider, duplicateProvider, activateProvider, testConnection } = useProviders()

const activePage = ref<'config' | 'sessions'>('config')
const currentView = ref<'list' | 'form'>('list')
const editingProvider = ref<Provider | null>(null)
const sessionDetailView = ref(false)
const sessionPageActive = computed(() => activePage.value === 'sessions')

function handleAdd() {
	editingProvider.value = null
	currentView.value = 'form'
}

function handleEdit(provider: Provider) {
	editingProvider.value = provider
	currentView.value = 'form'
}

function handleBack() {
	currentView.value = 'list'
	editingProvider.value = null
}

async function handleSave(formData: Omit<Provider, 'id' | 'is_active'>) {
	try {
		if (editingProvider.value) {
			await updateProvider({ ...editingProvider.value, ...formData })
			message.success('供应商已更新')
		} else {
			await addProvider(formData)
			message.success('供应商已添加')
		}
		currentView.value = 'list'
		editingProvider.value = null
	} catch (e) {
		message.error(String(e))
	}
}

async function handleDelete(id: string) {
	const provider = providers.value.find(p => p.id === id)
	if (provider?.is_active) {
		message.warning('不能删除当前激活的供应商')
		return
	}
	try {
		await deleteProvider(id)
		message.success('供应商已删除')
	} catch (e) {
		message.error(String(e))
	}
}

async function handleDuplicate(id: string) {
	try {
		const duplicate = await duplicateProvider(id)
		if (duplicate) {
			editingProvider.value = duplicate
			currentView.value = 'form'
			message.success('供应商已复制')
		}
	} catch (e) {
		message.error(String(e))
	}
}

async function handleActivate(provider: Provider) {
	try {
		await activateProvider(provider)
		message.success(`${provider.name} 已激活`)
	} catch (e) {
		message.error(String(e))
	}
}

async function handleTest(api_key: string, base_url: string) {
	return await testConnection(api_key, base_url)
}

function handleSessionDetailChange(isDetail: boolean) {
	sessionDetailView.value = isDetail
}

onMounted(loadProviders)
</script>

<template>
	<n-config-provider>
		<n-message-provider>
			<div class="flex flex-col w-full h-full bg-white font-sans">
				<!-- 顶部 Tab 栏 -->
				<div
					v-if="!sessionPageActive"
					class="flex items-center h-40px px-24px bg-white border-b border-gray-100 shrink-0 gap-4px"
				>
					<button
						class="flex items-center gap-6px px-12px py-6px rounded-6px text-13px transition-colors"
						:class="activePage === 'config' ? 'bg-indigo-500 text-white' : 'text-gray-600 hover:bg-gray-100'"
						@click="activePage = 'config'"
					>
						Config
					</button>
					<button
						class="flex items-center gap-6px px-12px py-6px rounded-6px text-13px transition-colors"
						:class="activePage === 'sessions' ? 'bg-indigo-500 text-white' : 'text-gray-600 hover:bg-gray-100'"
						@click="activePage = 'sessions'"
					>
						Sessions
					</button>
				</div>

				<!-- 内容区 -->
				<div class="flex-1 overflow-hidden">
					<ConfigListMain
						v-if="activePage === 'config' && currentView === 'list'"
						:providers="providers"
						:loading="loading"
						@add="handleAdd"
						@edit="handleEdit"
						@duplicate="handleDuplicate"
						@delete="handleDelete"
						@activate="handleActivate"
					/>
					<ProviderForm
						v-else-if="activePage === 'config' && currentView === 'form'"
						:provider="editingProvider"
						@save="handleSave"
						@cancel="handleBack"
						@test="handleTest"
					/>
					<SessionHistory
						v-else-if="activePage === 'sessions'"
						@detail-change="handleSessionDetailChange"
						@back="activePage = 'config'"
					/>
				</div>
			</div>
		</n-message-provider>
	</n-config-provider>
</template>
```

- [ ] **Step 2: 重构 ConfigListMain.vue**

替换 `src/components/ConfigListMain.vue` 的全部内容：

```vue
<script setup lang="ts">
import ProviderCard from './ProviderCard.vue'
import type { Provider } from '../types'

defineProps<{
	providers: Provider[]
	loading: boolean
}>()

defineEmits<{
	add: []
	edit: [provider: Provider]
	duplicate: [id: string]
	delete: [id: string]
	activate: [provider: Provider]
}>()
</script>

<template>
	<div class="flex flex-col w-full h-full bg-white font-sans">
		<header class="flex items-center justify-between h-56px px-24px bg-white border-b border-gray-100 shrink-0">
			<div class="flex items-center gap-16px">
				<span class="text-20px font-bold text-[#FF6B35]">CC-Desk</span>
			</div>
			<div class="flex items-center gap-8px">
				<button
					class="flex items-center justify-center w-36px h-36px border-none bg-[#FF6B35] rounded-full cursor-pointer hover:opacity-90 transition-opacity"
					@click="$emit('add')"
				>
					<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="#FFFFFF" stroke-width="2">
						<path d="M5 12h14" /><path d="M12 5v14" />
					</svg>
				</button>
			</div>
		</header>

		<main class="flex-1 py-16px px-24px bg-[#F8F9FA] overflow-y-auto">
			<div v-if="loading" class="flex items-center justify-center h-200px text-gray-400">
				加载中...
			</div>
			<div v-else-if="providers.length === 0" class="flex flex-col items-center justify-center h-200px text-gray-400">
				<p class="text-14px">暂无供应商配置</p>
				<p class="text-12px mt-8px">点击右上角 + 添加第一个供应商</p>
			</div>
			<div v-else class="flex flex-col gap-12px">
				<ProviderCard
					v-for="provider in providers"
					:key="provider.id"
					:provider="provider"
					@activate="$emit('activate', $event)"
					@edit="$emit('edit', $event)"
					@duplicate="$emit('duplicate', $event)"
					@delete="$emit('delete', $event)"
				/>
			</div>
		</main>
	</div>
</template>
```

- [ ] **Step 3: 编译验证**

Run: `pnpm type-check`
Expected: 无类型错误

- [ ] **Step 4: 运行开发服务器验证**

Run: `pnpm dev`
Expected: 应用正常启动，显示供应商列表页面

- [ ] **Step 5: Commit**

```bash
git add src/App.vue src/components/ConfigListMain.vue
git commit -m "feat: 重构 App.vue 和 ConfigListMain，集成供应商管理"
```

---

## Task 9: 集成测试和清理

**Files:**
- Modify: `src/App.vue` (修复编译问题)

- [ ] **Step 1: 修复 App.vue 中的未定义变量**

在 App.vue 的 `<script setup>` 中添加缺失的 computed：

```typescript
import { ref, computed, onMounted } from 'vue'

const sessionPageActive = computed(() => activePage.value === 'sessions')
```

- [ ] **Step 2: 运行完整构建**

Run: `pnpm build`
Expected: 构建成功

- [ ] **Step 3: 运行 Rust lint**

Run: `pnpm lint:rs`
Expected: 无警告

- [ ] **Step 4: 运行前端 lint**

Run: `pnpm lint`
Expected: 无错误

- [ ] **Step 5: Commit**

```bash
git add -A
git commit -m "chore: 修复编译问题，确保构建通过"
```
