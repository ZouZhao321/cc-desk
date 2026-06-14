# 会话历史功能实施计划

> **致代理工作者：** 必需子技能：使用 superpowers:subagent-driven-development（推荐）或 superpowers:executing-plans 逐任务执行本计划。步骤使用 checkbox（`- [ ]`）语法进行跟踪。

**目标：** 为 CC Desk 添加会话历史功能，支持浏览和标注 Claude Code 对话日志。

**架构：** Rust 后端扫描 `~/.claude/projects` 下的 JSONL 文件，解析会话元数据和完整对话内容，返回结构化数据给 Vue 前端。前端以项目分组的列表视图展示会话，支持详情视图浏览完整对话和添加备注标注。备注通过 `@tauri-apps/plugin-store` 持久化存储。

**技术栈：** Rust（Tauri 2）、Vue 3 组合式 API、TypeScript、UnoCSS、`@tauri-apps/plugin-store`

---

## 文件结构

| 文件 | 用途 |
|------|------|
| `src-tauri/src/commands.rs` | 添加 `SessionMeta`、`Message`、`ContentBlock` 类型和 `list_sessions`、`read_session`、`save_note`、`load_notes` 命令 |
| `src/types/index.ts` | 添加 `SessionMeta`、`Message`、`ContentBlock`、`MessageRole`、`SessionNote` 类型 |
| `src/composables/useSessionHistory.ts` | 用于加载会话、消息和管理备注的组合式函数 |
| `src/components/SessionCard.vue` | 单个会话卡片，显示时间、项目路径、备注状态 |
| `src/components/SessionDetail.vue` | 完整对话查看器，渲染所有消息类型 |
| `src/components/SessionNoteInput.vue` | 自动保存的备注输入框 |
| `src/components/SessionHistory.vue` | 主页面，编排列表和详情视图 |
| `src/App.vue` | 添加 Config 和 Sessions 之间的顶部 Tab 切换 |

---

### 任务 1：Rust 类型和命令

**文件：**
- 修改：`src-tauri/src/commands.rs`
- 修改：`src-tauri/src/lib.rs`

- [ ] **步骤 1：在 commands.rs 中添加会话类型**

在 `src-tauri/src/commands.rs` 末尾添加以下类型和函数：

```rust
use std::collections::HashMap;
use std::io::{BufRead, BufReader};

/// 会话元数据（列表页使用）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionMeta {
    pub session_id: String,
    pub project_path: String,
    pub started_at: Option<String>,
    pub message_count: usize,
    pub version: Option<String>,
}

/// 消息角色
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

/// 消息内容块
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ContentBlock {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "thinking")]
    Thinking { thinking: String },
    #[serde(rename = "tool_use")]
    ToolUse {
        id: String,
        name: String,
        input: serde_json::Value,
    },
    #[serde(rename = "tool_result")]
    ToolResult {
        tool_use_id: String,
        content: String,
    },
    #[serde(rename = "attachment")]
    Attachment {
        attachment_type: String,
        content: String,
    },
}

/// 完整消息（详情页使用）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub uuid: String,
    pub parent_uuid: Option<String>,
    pub timestamp: String,
    pub role: MessageRole,
    pub content: Vec<ContentBlock>,
    pub model: Option<String>,
    pub is_sidechain: bool,
}

/// 会话笔记
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SessionNote {
    pub session_id: String,
    pub note: String,
}

/// 项目目录名转原始路径
fn dir_name_to_project_path(dir_name: &str) -> String {
    // 目录名格式: C--Users-ZouZhao-Desktop-cc-desk
    // 转换为: C:\Users\ZouZhao\Desktop\cc-desk
    let parts: Vec<&str> = dir_name.splitn(2, "--").collect();
    if parts.len() < 2 {
        return dir_name.to_string();
    }
    let drive = parts[0];
    let rest = parts[1];
    let path = rest.replace('-', "\\");
    format!("{}:\\{}", drive, path)
}

/// 从 JSONL 第一行提取元数据
fn extract_meta_from_line(line: &str, session_id: &str, project_path: &str) -> Option<SessionMeta> {
    let json: serde_json::Value = serde_json::from_str(line).ok()?;
    let timestamp = json.get("timestamp")?.as_str()?.to_string();
    let version = json.get("version").and_then(|v| v.as_str()).map(|s| s.to_string());

    Some(SessionMeta {
        session_id: session_id.to_string(),
        project_path: project_path.to_string(),
        started_at: Some(timestamp),
        message_count: 0,
        version,
    })
}

/// 扫描所有项目目录，返回会话元数据列表
#[command]
pub fn list_sessions() -> Result<Vec<SessionMeta>, String> {
    let home = dirs::home_dir().ok_or_else(|| "无法确定用户主目录".to_string())?;
    let projects_dir = home.join(".claude").join("projects");

    if !projects_dir.exists() {
        return Ok(Vec::new());
    }

    let mut sessions = Vec::new();

    let entries = fs::read_dir(&projects_dir)
        .map_err(|e| format!("读取 projects 目录失败: {e}"))?;

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        let project_path = dir_name_to_project_path(dir_name);

        // 扫描目录下的 .jsonl 文件
        let jsonl_entries = fs::read_dir(&path).map_err(|e| format!("读取项目目录失败: {e}"))?;
        for jsonl_entry in jsonl_entries.flatten() {
            let jsonl_path = jsonl_entry.path();
            if jsonl_path.extension().and_then(|e| e.to_str()) != Some("jsonl") {
                continue;
            }

            let session_id = jsonl_path
                .file_stem()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();

            if session_id.is_empty() {
                continue;
            }

            // 读取前几行提取元数据
            let file = fs::File::open(&jsonl_path)
                .map_err(|e| format!("打开 JSONL 文件失败: {e}"))?;
            let reader = BufReader::new(file);
            let mut message_count = 0;
            let mut meta = None;

            for line in reader.lines().flatten() {
                if line.trim().is_empty() {
                    continue;
                }

                // 统计消息数量
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&line) {
                    let msg_type = json.get("type").and_then(|t| t.as_str()).unwrap_or("");
                    if msg_type == "user" || msg_type == "assistant" {
                        message_count += 1;
                    }

                    // 提取第一条消息作为元数据
                    if meta.is_none() {
                        meta = extract_meta_from_line(&line, &session_id, &project_path);
                    }
                }
            }

            if let Some(mut m) = meta {
                m.message_count = message_count;
                sessions.push(m);
            }
        }
    }

    // 按时间倒序排列
    sessions.sort_by(|a, b| {
        b.started_at
            .as_ref()
            .unwrap_or(&String::new())
            .cmp(a.started_at.as_ref().unwrap_or(&String::new()))
    });

    Ok(sessions)
}

/// 读取单个会话的完整消息列表
#[command]
pub fn read_session(session_id: String) -> Result<Vec<Message>, String> {
    let home = dirs::home_dir().ok_or_else(|| "无法确定用户主目录".to_string())?;
    let projects_dir = home.join(".claude").join("projects");

    // 查找包含该 session_id 的文件
    let entries = fs::read_dir(&projects_dir)
        .map_err(|e| format!("读取 projects 目录失败: {e}"))?;

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let jsonl_path = path.join(format!("{}.jsonl", session_id));
        if !jsonl_path.exists() {
            continue;
        }

        let file = fs::File::open(&jsonl_path)
            .map_err(|e| format!("打开 JSONL 文件失败: {e}"))?;
        let reader = BufReader::new(file);
        let mut messages = Vec::new();

        for line in reader.lines().flatten() {
            if line.trim().is_empty() {
                continue;
            }

            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&line) {
                let msg_type = json.get("type").and_then(|t| t.as_str()).unwrap_or("");
                let uuid = json.get("uuid").and_then(|u| u.as_str()).unwrap_or("").to_string();
                let parent_uuid = json.get("parentUuid").and_then(|u| u.as_str()).map(|s| s.to_string());
                let timestamp = json.get("timestamp").and_then(|t| t.as_str()).unwrap_or("").to_string();
                let is_sidechain = json.get("isSidechain").and_then(|b| b.as_bool()).unwrap_or(false);

                if msg_type == "user" {
                    let message = json.get("message");
                    let role = MessageRole::User;
                    let content = extract_content(message);
                    let model = None;

                    messages.push(Message {
                        uuid,
                        parent_uuid,
                        timestamp,
                        role,
                        content,
                        model,
                        is_sidechain,
                    });
                } else if msg_type == "assistant" {
                    let message = json.get("message");
                    let role = MessageRole::Assistant;
                    let content = extract_content(message);
                    let model = message
                        .and_then(|m| m.get("model"))
                        .and_then(|m| m.as_str())
                        .map(|s| s.to_string());

                    messages.push(Message {
                        uuid,
                        parent_uuid,
                        timestamp,
                        role,
                        content,
                        model,
                        is_sidechain,
                    });
                } else if msg_type == "attachment" {
                    let attachment = json.get("attachment");
                    let role = MessageRole::System;
                    let content = if let Some(att) = attachment {
                        let attachment_type = att.get("type").and_then(|t| t.as_str()).unwrap_or("unknown").to_string();
                        let text = att.get("content").and_then(|c| {
                            if let Some(s) = c.as_str() {
                                Some(s.to_string())
                            } else if let Some(arr) = c.as_array() {
                                Some(arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>().join("\n"))
                            } else {
                                None
                            }
                        }).unwrap_or_default();
                        vec![ContentBlock::Attachment { attachment_type, content: text }]
                    } else {
                        vec![]
                    };

                    messages.push(Message {
                        uuid,
                        parent_uuid,
                        timestamp,
                        role,
                        content,
                        model: None,
                        is_sidechain,
                    });
                }
            }
        }

        return Ok(messages);
    }

    Err(format!("未找到会话: {}", session_id))
}

/// 从 message.content 提取内容块
fn extract_content(message: Option<&serde_json::Value>) -> Vec<ContentBlock> {
    let Some(msg) = message else {
        return Vec::new();
    };

    let Some(content) = msg.get("content") else {
        return Vec::new();
    };

    let mut blocks = Vec::new();

    if let Some(arr) = content.as_array() {
        for item in arr {
            if let Some(block_type) = item.get("type").and_then(|t| t.as_str()) {
                match block_type {
                    "text" => {
                        if let Some(text) = item.get("text").and_then(|t| t.as_str()) {
                            blocks.push(ContentBlock::Text { text: text.to_string() });
                        }
                    }
                    "thinking" => {
                        if let Some(thinking) = item.get("thinking").and_then(|t| t.as_str()) {
                            blocks.push(ContentBlock::Thinking { thinking: thinking.to_string() });
                        }
                    }
                    "tool_use" => {
                        let id = item.get("id").and_then(|i| i.as_str()).unwrap_or("").to_string();
                        let name = item.get("name").and_then(|n| n.as_str()).unwrap_or("").to_string();
                        let input = item.get("input").cloned().unwrap_or(serde_json::json!({}));
                        blocks.push(ContentBlock::ToolUse { id, name, input });
                    }
                    "tool_result" => {
                        let tool_use_id = item.get("tool_use_id").and_then(|i| i.as_str()).unwrap_or("").to_string();
                        let content_text = item.get("content").and_then(|c| {
                            if let Some(s) = c.as_str() {
                                Some(s.to_string())
                            } else if let Some(arr) = c.as_array() {
                                Some(arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>().join("\n"))
                            } else {
                                None
                            }
                        }).unwrap_or_default();
                        blocks.push(ContentBlock::ToolResult { tool_use_id, content: content_text });
                    }
                    _ => {}
                }
            }
        }
    } else if let Some(text) = content.as_str() {
        blocks.push(ContentBlock::Text { text: text.to_string() });
    }

    blocks
}
```

- [ ] **步骤 2：在 lib.rs 中注册新命令**

更新 `src-tauri/src/lib.rs` 导入并注册新命令：

```rust
mod commands;

use commands::{get_settings_path, read_model_config, write_model_config, list_sessions, read_session};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            read_model_config,
            write_model_config,
            get_settings_path,
            list_sessions,
            read_session
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **步骤 3：验证 Rust 编译**

运行：`cd src-tauri && cargo check`
预期：无错误

- [ ] **步骤 4：提交**

```bash
git add src-tauri/src/commands.rs src-tauri/src/lib.rs
git commit -m "feat(rust): 添加会话历史扫描和解析命令"
```

---

### 任务 2：TypeScript 类型

**文件：**
- 修改：`src/types/index.ts`

- [ ] **步骤 1：添加会话类型**

在 `src/types/index.ts` 末尾追加以下类型：

```typescript
/** 会话元数据（列表页使用） */
export interface SessionMeta {
  session_id: string
  project_path: string
  started_at: string | null
  message_count: number
  version: string | null
}

/** 消息角色 */
export type MessageRole = 'user' | 'assistant' | 'system'

/** 消息内容块 */
export type ContentBlock =
  | { type: 'text'; text: string }
  | { type: 'thinking'; thinking: string }
  | { type: 'tool_use'; id: string; name: string; input: unknown }
  | { type: 'tool_result'; tool_use_id: string; content: string }
  | { type: 'attachment'; attachment_type: string; content: string }

/** 完整消息（详情页使用） */
export interface Message {
  uuid: string
  parent_uuid: string | null
  timestamp: string
  role: MessageRole
  content: ContentBlock[]
  model: string | null
  is_sidechain: boolean
}

/** 会话笔记 */
export interface SessionNote {
  session_id: string
  note: string
}

/** 笔记存储数据 */
export interface NoteStore {
  notes: Record<string, string>
}
```

- [ ] **步骤 2：验证 TypeScript 编译**

运行：`pnpm build`
预期：无类型错误

- [ ] **步骤 3：提交**

```bash
git add src/types/index.ts
git commit -m "feat(types): 添加会话历史相关类型定义"
```

---

### 任务 3：useSessionHistory 组合式函数

**文件：**
- 创建：`src/composables/useSessionHistory.ts`

- [ ] **步骤 1：创建组合式函数**

创建 `src/composables/useSessionHistory.ts`：

```typescript
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { load } from '@tauri-apps/plugin-store'
import type { SessionMeta, Message, NoteStore } from '../types'

const STORE_FILE = 'notes.json'

export function useSessionHistory() {
  const sessions = ref<SessionMeta[]>([])
  const currentMessages = ref<Message[]>([])
  const notes = ref<Record<string, string>>({})
  const loading = ref(false)
  const error = ref<string | null>(null)
  const selectedSessionId = ref<string | null>(null)

  const selectedSession = computed(() => {
    if (!selectedSessionId.value) return null
    return sessions.value.find(s => s.session_id === selectedSessionId.value) ?? null
  })

  const currentNote = computed(() => {
    if (!selectedSessionId.value) return ''
    return notes.value[selectedSessionId.value] ?? ''
  })

  async function loadSessions() {
    loading.value = true
    error.value = null
    try {
      const [sessionList, noteStore] = await Promise.all([
        invoke<SessionMeta[]>('list_sessions'),
        loadNotes()
      ])
      sessions.value = sessionList
      notes.value = noteStore
    } catch (e) {
      error.value = String(e)
    } finally {
      loading.value = false
    }
  }

  async function loadNotes(): Promise<Record<string, string>> {
    try {
      const store = await load(STORE_FILE, { defaults: {}, autoSave: false })
      const data = await store.get<NoteStore>('data')
      return data?.notes ?? {}
    } catch {
      return {}
    }
  }

  async function loadSession(sessionId: string) {
    loading.value = true
    error.value = null
    selectedSessionId.value = sessionId
    try {
      const messages = await invoke<Message[]>('read_session', { sessionId })
      currentMessages.value = messages
    } catch (e) {
      error.value = String(e)
    } finally {
      loading.value = false
    }
  }

  async function saveNote(sessionId: string, note: string) {
    notes.value[sessionId] = note
    try {
      const store = await load(STORE_FILE, { defaults: {}, autoSave: false })
      const data: NoteStore = { notes: notes.value }
      await store.set('data', data)
      await store.save()
    } catch (e) {
      error.value = String(e)
    }
  }

  function clearSession() {
    selectedSessionId.value = null
    currentMessages.value = []
  }

  function getNote(sessionId: string): string {
    return notes.value[sessionId] ?? ''
  }

  return {
    sessions,
    currentMessages,
    notes,
    loading,
    error,
    selectedSessionId,
    selectedSession,
    currentNote,
    loadSessions,
    loadSession,
    saveNote,
    clearSession,
    getNote
  }
}
```

- [ ] **步骤 2：验证 TypeScript 编译**

运行：`pnpm build`
预期：无类型错误

- [ ] **步骤 3：提交**

```bash
git add src/composables/useSessionHistory.ts
git commit -m "feat(composable): 添加 useSessionHistory 组合式函数"
```

---

### 任务 4：SessionCard 组件

**文件：**
- 创建：`src/components/SessionCard.vue`

- [ ] **步骤 1：创建组件**

创建 `src/components/SessionCard.vue`：

```vue
<script setup lang="ts">
import type { SessionMeta } from '../types'

defineProps<{
  session: SessionMeta
  note: string
}>()

defineEmits<{
  click: [sessionId: string]
}>()

function formatDate(dateStr: string | null): string {
  if (!dateStr) return '未知时间'
  const date = new Date(dateStr)
  const y = date.getFullYear()
  const m = String(date.getMonth() + 1).padStart(2, '0')
  const d = String(date.getDate()).padStart(2, '0')
  const h = String(date.getHours()).padStart(2, '0')
  const min = String(date.getMinutes()).padStart(2, '0')
  return `${y}-${m}-${d} ${h}:${min}`
}

function truncateNote(note: string, maxLen: number = 50): string {
  if (!note) return ''
  return note.length > maxLen ? note.slice(0, maxLen) + '...' : note
}
</script>

<template>
  <div
    class="flex flex-col gap-8px p-14px_18px bg-white rounded-10px border border-gray-200 cursor-pointer hover:border-blue-400 transition-colors"
    @click="$emit('click', session.session_id)"
  >
    <div class="flex items-center gap-8px text-13px">
      <span class="font-500 text-gray-900">{{ formatDate(session.started_at) }}</span>
      <span class="text-gray-400">·</span>
      <span class="font-mono text-11px" :class="session.project_path ? 'text-indigo-500' : 'text-gray-400'">
        {{ session.project_path || '未知路径' }}
      </span>
      <span class="text-gray-400">·</span>
      <span class="text-gray-500">{{ session.message_count }} messages</span>
    </div>
    <div
      v-if="note"
      class="flex items-center gap-6px px-10px py-6px bg-indigo-50 rounded-6px"
    >
      <svg class="w-12px h-12px text-indigo-500" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"/>
      </svg>
      <span class="text-12px text-indigo-600">{{ truncateNote(note) }}</span>
    </div>
    <div
      v-else
      class="flex items-center gap-6px px-10px py-6px bg-gray-100 rounded-6px"
    >
      <svg class="w-12px h-12px text-gray-400" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"/>
      </svg>
      <span class="text-12px text-gray-400">未添加备注</span>
    </div>
  </div>
</template>
```

- [ ] **步骤 2：验证 TypeScript 编译**

运行：`pnpm build`
预期：无类型错误

- [ ] **步骤 3：提交**

```bash
git add src/components/SessionCard.vue
git commit -m "feat(component): 添加 SessionCard 会话卡片组件"
```

---

### 任务 5：SessionNoteInput 组件

**文件：**
- 创建：`src/components/SessionNoteInput.vue`

- [ ] **步骤 1：创建组件**

创建 `src/components/SessionNoteInput.vue`：

```vue
<script setup lang="ts">
import { ref, watch } from 'vue'

const props = defineProps<{
  modelValue: string
  sessionId: string
}>()

const emit = defineEmits<{
  'update:modelValue': [value: string]
  save: [sessionId: string, note: string]
}>()

const localValue = ref(props.modelValue)
const saveTimer = ref<ReturnType<typeof setTimeout> | null>(null)

watch(() => props.modelValue, (newVal) => {
  localValue.value = newVal
})

function handleInput() {
  emit('update:modelValue', localValue.value)

  if (saveTimer.value) {
    clearTimeout(saveTimer.value)
  }

  saveTimer.value = setTimeout(() => {
    emit('save', props.sessionId, localValue.value)
  }, 500)
}

function handleBlur() {
  if (saveTimer.value) {
    clearTimeout(saveTimer.value)
  }
  emit('save', props.sessionId, localValue.value)
}
</script>

<template>
  <div class="flex flex-col gap-8px py-12px px-48px bg-white">
    <span class="text-12px font-500 text-gray-500">Note</span>
    <div class="flex items-center px-12px py-10px bg-gray-50 border border-gray-200 rounded-6px">
      <input
        v-model="localValue"
        type="text"
        class="flex-1 bg-transparent text-13px text-gray-900 outline-none placeholder:text-gray-400"
        placeholder="添加备注..."
        @input="handleInput"
        @blur="handleBlur"
      />
    </div>
  </div>
</template>
```

- [ ] **步骤 2：验证 TypeScript 编译**

运行：`pnpm build`
预期：无类型错误

- [ ] **步骤 3：提交**

```bash
git add src/components/SessionNoteInput.vue
git commit -m "feat(component): 添加 SessionNoteInput 备注输入组件"
```

---

### 任务 6：SessionDetail 组件

**文件：**
- 创建：`src/components/SessionDetail.vue`

- [ ] **步骤 1：创建组件**

创建 `src/components/SessionDetail.vue`：

```vue
<script setup lang="ts">
import { computed, ref } from 'vue'
import type { Message, ContentBlock } from '../types'

const props = defineProps<{
  messages: Message[]
  sessionInfo: {
    project_path: string
    started_at: string | null
    version: string | null
    message_count: number
  } | null
}>()

const expandedThinking = ref<Set<string>>(new Set())

function toggleThinking(uuid: string) {
  if (expandedThinking.value.has(uuid)) {
    expandedThinking.value.delete(uuid)
  } else {
    expandedThinking.value.add(uuid)
  }
}

function formatTime(timestamp: string): string {
  const date = new Date(timestamp)
  const h = String(date.getHours()).padStart(2, '0')
  const m = String(date.getMinutes()).padStart(2, '0')
  const s = String(date.getSeconds()).padStart(2, '0')
  return `${h}:${m}:${s}`
}

function getRoleLabel(role: string): string {
  switch (role) {
    case 'user': return 'USER'
    case 'assistant': return 'ASSISTANT'
    case 'system': return 'SYSTEM'
    default: return role.toUpperCase()
  }
}

function getRoleColor(role: string): string {
  switch (role) {
    case 'user': return 'text-gray-500'
    case 'assistant': return 'text-indigo-600'
    case 'system': return 'text-gray-400'
    default: return 'text-gray-500'
  }
}

function getTextBlocks(content: ContentBlock[]): string {
  return content
    .filter((b): b is { type: 'text'; text: string } => b.type === 'text')
    .map(b => b.text)
    .join('\n')
}

function getToolUseBlocks(content: ContentBlock[]): { id: string; name: string; input: unknown }[] {
  return content
    .filter((b): b is { type: 'tool_use'; id: string; name: string; input: unknown } => b.type === 'tool_use')
}

function getToolResultBlocks(content: ContentBlock[]): { tool_use_id: string; content: string }[] {
  return content
    .filter((b): b is { type: 'tool_result'; tool_use_id: string; content: string } => b.type === 'tool_result')
}

function getThinkingBlocks(content: ContentBlock[]): string[] {
  return content
    .filter((b): b is { type: 'thinking'; thinking: string } => b.type === 'thinking')
    .map(b => b.thinking)
}

function getAttachmentBlocks(content: ContentBlock[]): { attachment_type: string; content: string }[] {
  return content
    .filter((b): b is { type: 'attachment'; attachment_type: string; content: string } => b.type === 'attachment')
}

const headerInfo = computed(() => {
  if (!props.sessionInfo) return null
  const parts = [props.sessionInfo.project_path]
  if (props.sessionInfo.started_at) {
    const date = new Date(props.sessionInfo.started_at)
    parts.push(date.toLocaleString())
  }
  if (props.sessionInfo.version) parts.push(`v${props.sessionInfo.version}`)
  parts.push(`${props.sessionInfo.message_count} messages`)
  return parts.join(' · ')
})
</script>

<template>
  <div class="flex flex-col w-full h-full bg-white">
    <header class="flex items-center h-56px px-24px bg-white border-b border-gray-100 shrink-0 gap-16px">
      <div class="flex items-center gap-8px text-gray-500 cursor-pointer hover:text-gray-700">
        <svg class="w-16px h-16px" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M19 12H5M12 19l-7-7 7-7"/>
        </svg>
        <span class="text-14px">Back</span>
      </div>
      <div v-if="headerInfo" class="flex-1 text-11px font-mono text-gray-400">
        {{ headerInfo }}
      </div>
    </header>

    <div class="h-1px bg-gray-200 shrink-0"></div>

    <main class="flex-1 overflow-y-auto py-20px px-48px">
      <div class="flex flex-col gap-20px">
        <div v-for="msg in messages" :key="msg.uuid" class="flex flex-col gap-6px">
          <div class="flex items-center gap-8px text-11px font-mono">
            <span :class="getRoleColor(msg.role)">{{ getRoleLabel(msg.role) }}</span>
            <span class="text-gray-400">·</span>
            <span class="text-gray-400">{{ formatTime(msg.timestamp) }}</span>
            <span v-if="msg.model" class="text-gray-400">· {{ msg.model }}</span>
          </div>

          <div v-if="getTextBlocks(msg.content)" class="text-14px text-gray-900 leading-relaxed whitespace-pre-wrap">
            {{ getTextBlocks(msg.content) }}
          </div>

          <div v-for="tool in getToolUseBlocks(msg.content)" :key="tool.id" class="flex flex-col gap-6px">
            <div class="text-11px font-mono text-indigo-600">TOOL USE · {{ tool.name }}</div>
            <div class="p-12px bg-gray-50 border border-gray-200 rounded-6px font-mono text-12px text-gray-700">
              {{ JSON.stringify(tool.input, null, 2) }}
            </div>
          </div>

          <div v-for="result in getToolResultBlocks(msg.content)" :key="result.tool_use_id" class="flex flex-col gap-6px">
            <div class="text-11px font-mono text-gray-400">TOOL RESULT</div>
            <div class="p-12px bg-gray-50 border border-gray-200 rounded-6px font-mono text-12px text-gray-700 max-h-200px overflow-y-auto">
              {{ result.content }}
            </div>
          </div>

          <div v-for="(thinking, idx) in getThinkingBlocks(msg.content)" :key="idx">
            <div
              class="flex items-center gap-8px px-12px py-8px bg-gray-50 rounded-6px cursor-pointer"
              @click="toggleThinking(msg.uuid)"
            >
              <svg class="w-14px h-14px text-gray-400" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M12 2a7 7 0 0 1 7 7c0 2.5-1.5 4.5-3 6-.5.5-1 1.5-1 2.5V19a1 1 0 0 1-1 1h-4a1 1 0 0 1-1-1v-1.5c0-1-.5-2-1-2.5-1.5-1.5-3-3.5-3-6a7 7 0 0 1 7-7z"/>
                <line x1="9" y1="21" x2="15" y2="21"/>
              </svg>
              <span class="text-12px text-gray-400">
                {{ expandedThinking.has(msg.uuid) ? 'Collapse' : 'Thinking...' }}
              </span>
            </div>
            <div v-if="expandedThinking.has(msg.uuid)" class="mt-8px p-12px bg-gray-50 border border-gray-200 rounded-6px text-13px text-gray-600 leading-relaxed whitespace-pre-wrap">
              {{ thinking }}
            </div>
          </div>

          <div v-for="(att, idx) in getAttachmentBlocks(msg.content)" :key="idx" class="text-12px text-gray-400">
            [{{ att.attachment_type }}]
          </div>
        </div>
      </div>
    </main>
  </div>
</template>
```

- [ ] **步骤 2：验证 TypeScript 编译**

运行：`pnpm build`
预期：无类型错误

- [ ] **步骤 3：提交**

```bash
git add src/components/SessionDetail.vue
git commit -m "feat(component): 添加 SessionDetail 对话详情组件"
```

---

### 任务 7：SessionHistory 主页面

**文件：**
- 创建：`src/components/SessionHistory.vue`

- [ ] **步骤 1：创建组件**

创建 `src/components/SessionHistory.vue`：

```vue
<script setup lang="ts">
import { computed } from 'vue'
import { useSessionHistory } from '../composables/useSessionHistory'
import SessionCard from './SessionCard.vue'
import SessionDetail from './SessionDetail.vue'
import SessionNoteInput from './SessionNoteInput.vue'

const {
  sessions,
  currentMessages,
  loading,
  error,
  selectedSessionId,
  selectedSession,
  currentNote,
  loadSessions,
  loadSession,
  saveNote,
  clearSession,
  getNote
} = useSessionHistory()

const groupedSessions = computed(() => {
  const groups: Record<string, typeof sessions.value> = {}
  for (const session of sessions.value) {
    const path = session.project_path || '未知项目'
    if (!groups[path]) groups[path] = []
    groups[path].push(session)
  }
  return Object.entries(groups)
})

const noteInputValue = computed({
  get: () => currentNote.value,
  set: (val: string) => {
    if (selectedSessionId.value) {
      saveNote(selectedSessionId.value, val)
    }
  }
})

function handleCardClick(sessionId: string) {
  loadSession(sessionId)
}

function handleBack() {
  clearSession()
}

function handleRefresh() {
  loadSessions()
}

function handleNoteSave(sessionId: string, note: string) {
  saveNote(sessionId, note)
}

// 初始加载
loadSessions()
</script>

<template>
  <div class="flex flex-col w-full h-full bg-white font-sans">
    <header class="flex items-center justify-between h-56px px-24px bg-white border-b border-gray-100 shrink-0">
      <div class="flex items-center gap-16px">
        <span class="text-20px font-bold text-[#FF6B35]">CC-Desk</span>
        <div class="flex-1"></div>
        <button
          class="flex items-center gap-6px px-12px py-6px bg-gray-100 rounded-6px text-12px text-gray-600 hover:bg-gray-200 transition-colors"
          @click="handleRefresh"
        >
          <svg class="w-14px h-14px" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M23 4v6h-6M1 20v-6h6"/>
            <path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"/>
          </svg>
          Refresh
        </button>
      </div>
    </header>

    <!-- 列表视图 -->
    <template v-if="!selectedSessionId">
      <main class="flex-1 py-16px px-24px bg-[#F8F9FA] overflow-y-auto">
        <div v-if="loading && sessions.length === 0" class="flex items-center justify-center py-48px text-gray-400">
          加载中...
        </div>
        <div v-else-if="error" class="flex flex-col items-center gap-12px py-48px">
          <span class="text-red-500">{{ error }}</span>
          <button class="text-13px text-indigo-600 hover:underline" @click="handleRefresh">重试</button>
        </div>
        <div v-else-if="sessions.length === 0" class="flex items-center justify-center py-48px text-gray-400">
          暂无会话记录
        </div>
        <div v-else class="flex flex-col gap-24px">
          <div v-for="[projectPath, projectSessions] in groupedSessions" :key="projectPath" class="flex flex-col gap-12px">
            <div class="flex items-center gap-8px px-0 py-8px">
              <svg class="w-14px h-14px text-indigo-500" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
              </svg>
              <span class="text-13px font-600 text-gray-700">{{ projectPath }}</span>
              <span class="text-12px text-gray-400">{{ projectSessions.length }} sessions</span>
            </div>
            <div class="flex flex-col gap-12px">
              <SessionCard
                v-for="session in projectSessions"
                :key="session.session_id"
                :session="session"
                :note="getNote(session.session_id)"
                @click="handleCardClick"
              />
            </div>
          </div>
        </div>
      </main>
    </template>

    <!-- 详情视图 -->
    <template v-else>
      <SessionDetail
        :messages="currentMessages"
        :session-info="selectedSession ? {
          project_path: selectedSession.project_path,
          started_at: selectedSession.started_at,
          version: selectedSession.version,
          message_count: selectedSession.message_count
        } : null"
        @back="handleBack"
      />
      <SessionNoteInput
        v-if="selectedSessionId"
        :model-value="noteInputValue"
        :session-id="selectedSessionId"
        @update:model-value="noteInputValue = $event"
        @save="handleNoteSave"
      />
    </template>
  </div>
</template>
```

- [ ] **步骤 2：验证 TypeScript 编译**

运行：`pnpm build`
预期：无类型错误

- [ ] **步骤 3：提交**

```bash
git add src/components/SessionHistory.vue
git commit -m "feat(component): 添加 SessionHistory 主页面组件"
```

---

### 任务 8：更新 App.vue 导航

**文件：**
- 修改：`src/App.vue`

- [ ] **步骤 1：添加 Tab 导航**

更新 `src/App.vue` 添加顶部 Tab 切换：

1. 添加 SessionHistory 导入：

```typescript
import SessionHistory from './components/SessionHistory.vue'
```

2. 添加页面状态：

```typescript
const activePage = ref<'config' | 'sessions'>('config')
```

3. 更新模板，添加 Tab 栏和条件渲染：

替换模板为：

```vue
<template>
  <n-config-provider>
    <n-message-provider>
      <div class="flex flex-col w-full h-full bg-white font-sans">
        <!-- 顶部 Tab 栏 -->
        <div class="flex items-center h-40px px-24px bg-white border-b border-gray-100 shrink-0 gap-4px">
          <button
            class="flex items-center gap-6px px-12px py-6px rounded-6px text-13px transition-colors"
            :class="activePage === 'config' ? 'bg-indigo-500 text-white' : 'text-gray-600 hover:bg-gray-100'"
            @click="activePage = 'config'"
          >
            <svg class="w-14px h-14px" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="12" cy="12" r="3"/>
              <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/>
            </svg>
            Config
          </button>
          <button
            class="flex items-center gap-6px px-12px py-6px rounded-6px text-13px transition-colors"
            :class="activePage === 'sessions' ? 'bg-indigo-500 text-white' : 'text-gray-600 hover:bg-gray-100'"
            @click="activePage = 'sessions'"
          >
            <svg class="w-14px h-14px" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"/>
            </svg>
            Sessions
          </button>
        </div>

        <!-- 内容区 -->
        <div class="flex-1 overflow-hidden">
          <ConfigListMain
            v-if="activePage === 'config' && currentView === 'list'"
            :scopes="scopes"
            @select="handleSelectScope"
            @add="handleAdd"
            @settings="handleSettings"
            @sync="handleSync"
          />
          <ConfigDetail
            v-else-if="activePage === 'config' && currentView === 'detail'"
            :config-name="configName"
            :active-tab="activeTab"
            :items="currentItems"
            :mode="activeScope === 'global' ? 'global' : 'project'"
            :inherited-items="currentInherited"
            :overrides="activeTab === 'mcp' ? projectOverrides : undefined"
            :projects="activeTab === 'project' ? projects : undefined"
            :inherited-count="currentInheritedCount"
            @back="handleBack"
            @tab-change="handleTabChange"
            @toggle="handleToggle"
            @save="handleSave"
            @reset="handleReset"
            @override-update="handleOverrideUpdate"
            @override-remove="handleOverrideRemove"
          />
          <SessionHistory v-else-if="activePage === 'sessions'" />
        </div>
      </div>
    </n-message-provider>
  </n-config-provider>
</template>
```

- [ ] **步骤 2：验证 TypeScript 编译**

运行：`pnpm build`
预期：无类型错误

- [ ] **步骤 3：提交**

```bash
git add src/App.vue
git commit -m "feat(app): 添加 Config/Sessions 顶部 Tab 导航"
```

---

### 任务 9：最终验证

- [ ] **步骤 1：运行完整构建**

运行：`pnpm build`
预期：无错误

- [ ] **步骤 2：运行 lint**

运行：`pnpm lint`
预期：无错误

- [ ] **步骤 3：运行 Rust lint**

运行：`pnpm lint:rs`
预期：无错误

- [ ] **步骤 4：开发模式测试**

运行：`pnpm tauri dev`
预期：应用启动，Tab 切换正常，会话列表加载，详情视图显示消息

- [ ] **步骤 5：最终提交**

```bash
git add -A
git commit -m "feat: 完成会话历史功能 MVP"
```
