use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::command;

/// 测试 API 连通性（可达性探测）
/// 任何 HTTP 响应均视为连接成功，仅网络层错误（DNS/拒绝连接/TLS/超时）视为失败
#[command]
pub async fn test_connection(base_url: String) -> Result<String, String> {
    let url = base_url.trim_end_matches('/').to_string();
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("accept", "*/*")
        .header("accept-encoding", "identity")
        .timeout(std::time::Duration::from_secs(8))
        .send()
        .await
        .map_err(|e| format!("连接失败: {e}"))?;

    let status = response.status().as_u16();
    Ok(format!("连接成功 (HTTP {status})"))
}

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

/// 供应商配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Provider {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    let content = fs::read_to_string(&path).map_err(|e| format!("读取 settings.json 失败: {e}"))?;
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
    let content = fs::read_to_string(&path).map_err(|e| format!("读取 settings.json 失败: {e}"))?;
    let mut json: serde_json::Value =
        serde_json::from_str(&content).map_err(|e| format!("解析 settings.json 失败: {e}"))?;

    // 确保 env 对象存在
    if json.get("env").is_none() {
        json["env"] = serde_json::json!({});
    }
    let env = json["env"].as_object_mut().ok_or("env 不是对象类型")?;

    // 只写入模型相关字段
    env.insert("ANTHROPIC_AUTH_TOKEN".into(), serde_json::json!(config.auth_token));
    env.insert("ANTHROPIC_BASE_URL".into(), serde_json::json!(config.base_url));
    // env.ANTHROPIC_MODEL 根据 model 角色选择派生对应模型 ID
    let derived_model_id = derive_anthropic_model(&config.model, &config.haiku_id, &config.sonnet_id, &config.opus_id);
    env.insert("ANTHROPIC_MODEL".into(), serde_json::json!(derived_model_id));
    env.insert(
        "ANTHROPIC_REASONING_MODEL".into(),
        serde_json::json!(config.reasoning_model),
    );
    env.insert(
        "ANTHROPIC_DEFAULT_HAIKU_MODEL".into(),
        serde_json::json!(config.haiku_id),
    );
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

fn settings_path() -> Result<PathBuf, String> {
    let home = dirs::home_dir().ok_or_else(|| "无法确定用户主目录".to_string())?;
    Ok(home.join(".claude").join("settings.json"))
}

/// 返回 ~/.cc-desk/providers.json 的路径（自动创建目录）
fn providers_path() -> Result<PathBuf, String> {
    let home = dirs::home_dir().ok_or_else(|| "无法确定用户主目录".to_string())?;
    let dir = home.join(".cc-desk");
    fs::create_dir_all(&dir).map_err(|e| format!("创建 .cc-desk 目录失败: {e}"))?;
    Ok(dir.join("providers.json"))
}

fn env_str(env: &serde_json::Value, key: &str) -> String {
    env.get(key).and_then(|v| v.as_str()).unwrap_or_default().to_string()
}

fn json_str(json: &serde_json::Value, key: &str) -> String {
    json.get(key).and_then(|v| v.as_str()).unwrap_or_default().to_string()
}

/// 根据角色名派生 ANTHROPIC_MODEL 值
fn derive_anthropic_model(main_model: &str, haiku: &str, sonnet: &str, opus: &str) -> String {
    match main_model {
        "haiku" => haiku.to_string(),
        "sonnet" => sonnet.to_string(),
        "opus" => opus.to_string(),
        _ => main_model.to_string(),
    }
}

/// 读取所有供应商配置
#[command]
pub fn list_providers() -> Result<Vec<Provider>, String> {
    let path = providers_path()?;
    if !path.exists() {
        return Ok(Vec::new());
    }
    let content = fs::read_to_string(&path).map_err(|e| format!("读取 providers.json 失败: {e}"))?;
    let store: ProviderStore = serde_json::from_str(&content).map_err(|e| format!("解析 providers.json 失败: {e}"))?;
    Ok(store.providers)
}

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
    let derived_model_id = derive_anthropic_model(
        &provider.main_model,
        &provider.haiku_model,
        &provider.sonnet_model,
        &provider.opus_model,
    );
    env.insert("ANTHROPIC_MODEL".into(), serde_json::json!(derived_model_id));

    env.insert(
        "ANTHROPIC_REASONING_MODEL".into(),
        serde_json::json!(provider.sub_agent_model),
    );
    env.insert(
        "ANTHROPIC_DEFAULT_HAIKU_MODEL".into(),
        serde_json::json!(provider.haiku_model),
    );
    env.insert(
        "ANTHROPIC_DEFAULT_SONNET_MODEL".into(),
        serde_json::json!(provider.sonnet_model),
    );
    env.insert(
        "ANTHROPIC_DEFAULT_OPUS_MODEL".into(),
        serde_json::json!(provider.opus_model),
    );
    env.insert(
        "CLAUDE_CODE_EFFORT_LEVEL".into(),
        serde_json::json!(provider.reasoning_level),
    );

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
    ToolResult { tool_use_id: String, content: String },
    #[serde(rename = "attachment")]
    Attachment { attachment_type: String, content: String },
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
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SessionNote {
    pub session_id: String,
    pub note: String,
}

/// 从 JSONL 条目中提取 cwd 字段
fn extract_cwd_from_line(line: &str) -> Option<String> {
    let json: serde_json::Value = serde_json::from_str(line).ok()?;
    json.get("cwd").and_then(|v| v.as_str()).map(|s| s.to_string())
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

    let entries = fs::read_dir(&projects_dir).map_err(|e| format!("读取 projects 目录失败: {e}"))?;

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

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
            let file = fs::File::open(&jsonl_path).map_err(|e| format!("打开 JSONL 文件失败: {e}"))?;
            let reader = BufReader::new(file);
            let mut message_count = 0;
            let mut meta = None;
            let mut project_path = None;

            for line in reader.lines().map_while(Result::ok) {
                if line.trim().is_empty() {
                    continue;
                }

                // 从 cwd 字段获取真实项目路径
                if project_path.is_none() {
                    project_path = extract_cwd_from_line(&line);
                }

                // 统计消息数量
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&line) {
                    let msg_type = json.get("type").and_then(|t| t.as_str()).unwrap_or("");
                    if msg_type == "user" || msg_type == "assistant" {
                        message_count += 1;
                    }

                    // 提取第一条消息作为元数据
                    if meta.is_none() {
                        if let Some(ref cwd) = project_path {
                            meta = extract_meta_from_line(&line, &session_id, cwd);
                        }
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
    let entries = fs::read_dir(&projects_dir).map_err(|e| format!("读取 projects 目录失败: {e}"))?;

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let jsonl_path = path.join(format!("{}.jsonl", session_id));
        if !jsonl_path.exists() {
            continue;
        }

        let file = fs::File::open(&jsonl_path).map_err(|e| format!("打开 JSONL 文件失败: {e}"))?;
        let reader = BufReader::new(file);
        let mut messages = Vec::new();

        for line in reader.lines().map_while(Result::ok) {
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
                        let attachment_type = att
                            .get("type")
                            .and_then(|t| t.as_str())
                            .unwrap_or("unknown")
                            .to_string();
                        let text = att
                            .get("content")
                            .and_then(|c| {
                                if let Some(s) = c.as_str() {
                                    Some(s.to_string())
                                } else if let Some(arr) = c.as_array() {
                                    Some(arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>().join("\n"))
                                } else {
                                    None
                                }
                            })
                            .unwrap_or_default();
                        vec![ContentBlock::Attachment {
                            attachment_type,
                            content: text,
                        }]
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

/// 获取会话的最后一条用户消息
#[command]
pub fn get_session_last_message(session_id: String) -> Result<Option<String>, String> {
    let home = dirs::home_dir().ok_or_else(|| "无法确定用户主目录".to_string())?;
    let projects_dir = home.join(".claude").join("projects");

    let entries = fs::read_dir(&projects_dir).map_err(|e| format!("读取 projects 目录失败: {e}"))?;

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let jsonl_path = path.join(format!("{}.jsonl", session_id));
        if !jsonl_path.exists() {
            continue;
        }

        let file = fs::File::open(&jsonl_path).map_err(|e| format!("打开 JSONL 文件失败: {e}"))?;
        let reader = BufReader::new(file);

        let mut last_user_message: Option<String> = None;

        for line in reader.lines().map_while(Result::ok) {
            if line.trim().is_empty() {
                continue;
            }

            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&line) {
                let msg_type = json.get("type").and_then(|t| t.as_str()).unwrap_or("");
                if msg_type == "user" {
                    if let Some(text) = extract_last_message_text(&json) {
                        last_user_message = Some(text);
                    }
                }
            }
        }

        return Ok(last_user_message);
    }

    Ok(None)
}

/// 从用户消息 JSON 中提取消息文本
fn extract_last_message_text(json: &serde_json::Value) -> Option<String> {
    let message = json.get("message")?;
    let content = message.get("content")?;

    if let Some(arr) = content.as_array() {
        for item in arr {
            if let Some(block_type) = item.get("type").and_then(|t| t.as_str()) {
                if block_type == "text" {
                    return item.get("text").and_then(|t| t.as_str()).map(|s| s.to_string());
                }
            }
        }
    } else if let Some(text) = content.as_str() {
        return Some(text.to_string());
    }

    None
}

/// 笔记存储数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NoteStore {
    pub notes: std::collections::HashMap<String, String>,
}

/// 返回 ~/.cc-desk/annotations.json 的路径
fn annotations_path() -> Result<PathBuf, String> {
    let home = dirs::home_dir().ok_or_else(|| "无法确定用户主目录".to_string())?;
    let dir = home.join(".cc-desk");
    fs::create_dir_all(&dir).map_err(|e| format!("创建 .cc-desk 目录失败: {e}"))?;
    Ok(dir.join("annotations.json"))
}

/// 加载所有备注
#[command]
pub fn load_annotations() -> Result<std::collections::HashMap<String, String>, String> {
    let path = annotations_path()?;
    if !path.exists() {
        return Ok(std::collections::HashMap::new());
    }
    let content = fs::read_to_string(&path).map_err(|e| format!("读取 annotations.json 失败: {e}"))?;
    let store: NoteStore = serde_json::from_str(&content).map_err(|e| format!("解析 annotations.json 失败: {e}"))?;
    Ok(store.notes)
}

/// 保存单条备注（原子写入）
#[command]
pub fn save_annotation(session_id: String, note: String) -> Result<(), String> {
    let path = annotations_path()?;

    // 读取现有备注
    let mut notes: std::collections::HashMap<String, String> = if path.exists() {
        let content = fs::read_to_string(&path).map_err(|e| format!("读取 annotations.json 失败: {e}"))?;
        let store: NoteStore =
            serde_json::from_str(&content).map_err(|e| format!("解析 annotations.json 失败: {e}"))?;
        store.notes
    } else {
        std::collections::HashMap::new()
    };

    notes.insert(session_id, note);
    let store = NoteStore { notes };
    let serialized = serde_json::to_string_pretty(&store).map_err(|e| format!("序列化失败: {e}"))?;

    // 原子写入: .tmp → rename
    let tmp_path = path.with_extension("json.tmp");
    fs::write(&tmp_path, &serialized).map_err(|e| format!("写入临时文件失败: {e}"))?;
    fs::rename(&tmp_path, &path).map_err(|e| format!("重命名临时文件失败: {e}"))?;

    Ok(())
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
                            blocks.push(ContentBlock::Thinking {
                                thinking: thinking.to_string(),
                            });
                        }
                    }
                    "tool_use" => {
                        let id = item.get("id").and_then(|i| i.as_str()).unwrap_or("").to_string();
                        let name = item.get("name").and_then(|n| n.as_str()).unwrap_or("").to_string();
                        let input = item.get("input").cloned().unwrap_or(serde_json::json!({}));
                        blocks.push(ContentBlock::ToolUse { id, name, input });
                    }
                    "tool_result" => {
                        let tool_use_id = item
                            .get("tool_use_id")
                            .and_then(|i| i.as_str())
                            .unwrap_or("")
                            .to_string();
                        let content_text = item
                            .get("content")
                            .and_then(|c| {
                                if let Some(s) = c.as_str() {
                                    Some(s.to_string())
                                } else if let Some(arr) = c.as_array() {
                                    Some(arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>().join("\n"))
                                } else {
                                    None
                                }
                            })
                            .unwrap_or_default();
                        blocks.push(ContentBlock::ToolResult {
                            tool_use_id,
                            content: content_text,
                        });
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

#[cfg(test)]
mod tests {
    use super::*;

    // ── extract_cwd_from_line ──

    #[test]
    fn extract_cwd_normal() {
        let line = r#"{"type":"attachment","cwd":"C:\\Users\\ZouZhao\\Desktop\\cc-desk","sessionId":"abc"}"#;
        let cwd = extract_cwd_from_line(line);
        assert_eq!(cwd.as_deref(), Some("C:\\Users\\ZouZhao\\Desktop\\cc-desk"));
    }

    #[test]
    fn extract_cwd_missing() {
        let line = r#"{"type":"user","message":{}}"#;
        assert!(extract_cwd_from_line(line).is_none());
    }

    #[test]
    fn extract_cwd_invalid_json() {
        assert!(extract_cwd_from_line("not json").is_none());
    }

    // ── extract_meta_from_line ──

    #[test]
    fn extract_meta_with_version() {
        let line = r#"{"timestamp":"2026-06-14T10:00:00Z","version":"1.0.30"}"#;
        let meta = extract_meta_from_line(line, "sess-1", "my-project").unwrap();
        assert_eq!(meta.session_id, "sess-1");
        assert_eq!(meta.project_path, "my-project");
        assert_eq!(meta.started_at.as_deref(), Some("2026-06-14T10:00:00Z"));
        assert_eq!(meta.version.as_deref(), Some("1.0.30"));
        assert_eq!(meta.message_count, 0);
    }

    #[test]
    fn extract_meta_without_version() {
        let line = r#"{"timestamp":"2026-06-14T10:00:00Z"}"#;
        let meta = extract_meta_from_line(line, "sess-2", "proj").unwrap();
        assert_eq!(meta.version, None);
    }

    #[test]
    fn extract_meta_invalid_json() {
        assert!(extract_meta_from_line("not json", "s", "p").is_none());
    }

    #[test]
    fn extract_meta_missing_timestamp() {
        let line = r#"{"version":"1.0"}"#;
        assert!(extract_meta_from_line(line, "s", "p").is_none());
    }

    // ── extract_content ──

    #[test]
    fn extract_content_none() {
        assert!(extract_content(None).is_empty());
    }

    #[test]
    fn extract_content_no_content_field() {
        let msg = serde_json::json!({"role": "user"});
        assert!(extract_content(Some(&msg)).is_empty());
    }

    #[test]
    fn extract_content_text_block() {
        let msg = serde_json::json!({
            "content": [{"type": "text", "text": "hello"}]
        });
        let blocks = extract_content(Some(&msg));
        assert_eq!(blocks.len(), 1);
        match &blocks[0] {
            ContentBlock::Text { text } => assert_eq!(text, "hello"),
            _ => panic!("expected Text block"),
        }
    }

    #[test]
    fn extract_content_thinking_block() {
        let msg = serde_json::json!({
            "content": [{"type": "thinking", "thinking": "let me think..."}]
        });
        let blocks = extract_content(Some(&msg));
        assert_eq!(blocks.len(), 1);
        match &blocks[0] {
            ContentBlock::Thinking { thinking } => assert_eq!(thinking, "let me think..."),
            _ => panic!("expected Thinking block"),
        }
    }

    #[test]
    fn extract_content_tool_use_block() {
        let msg = serde_json::json!({
            "content": [{
                "type": "tool_use",
                "id": "tu-1",
                "name": "Read",
                "input": {"file_path": "/tmp/test.rs"}
            }]
        });
        let blocks = extract_content(Some(&msg));
        assert_eq!(blocks.len(), 1);
        match &blocks[0] {
            ContentBlock::ToolUse { id, name, input } => {
                assert_eq!(id, "tu-1");
                assert_eq!(name, "Read");
                assert_eq!(input["file_path"], "/tmp/test.rs");
            }
            _ => panic!("expected ToolUse block"),
        }
    }

    #[test]
    fn extract_content_tool_result_string() {
        let msg = serde_json::json!({
            "content": [{
                "type": "tool_result",
                "tool_use_id": "tu-1",
                "content": "file contents"
            }]
        });
        let blocks = extract_content(Some(&msg));
        assert_eq!(blocks.len(), 1);
        match &blocks[0] {
            ContentBlock::ToolResult { tool_use_id, content } => {
                assert_eq!(tool_use_id, "tu-1");
                assert_eq!(content, "file contents");
            }
            _ => panic!("expected ToolResult block"),
        }
    }

    #[test]
    fn extract_content_tool_result_array() {
        let msg = serde_json::json!({
            "content": [{
                "type": "tool_result",
                "tool_use_id": "tu-2",
                "content": ["line1", "line2"]
            }]
        });
        let blocks = extract_content(Some(&msg));
        match &blocks[0] {
            ContentBlock::ToolResult { content, .. } => assert_eq!(content, "line1\nline2"),
            _ => panic!("expected ToolResult block"),
        }
    }

    #[test]
    fn extract_content_plain_string() {
        let msg = serde_json::json!({
            "content": "just a string"
        });
        let blocks = extract_content(Some(&msg));
        assert_eq!(blocks.len(), 1);
        match &blocks[0] {
            ContentBlock::Text { text } => assert_eq!(text, "just a string"),
            _ => panic!("expected Text block"),
        }
    }

    #[test]
    fn extract_content_mixed_blocks() {
        let msg = serde_json::json!({
            "content": [
                {"type": "text", "text": "hello"},
                {"type": "thinking", "thinking": "hmm"},
                {"type": "tool_use", "id": "t1", "name": "Bash", "input": {}}
            ]
        });
        let blocks = extract_content(Some(&msg));
        assert_eq!(blocks.len(), 3);
        assert!(matches!(&blocks[0], ContentBlock::Text { .. }));
        assert!(matches!(&blocks[1], ContentBlock::Thinking { .. }));
        assert!(matches!(&blocks[2], ContentBlock::ToolUse { .. }));
    }

    #[test]
    fn extract_content_unknown_type_ignored() {
        let msg = serde_json::json!({
            "content": [
                {"type": "text", "text": "keep"},
                {"type": "unknown_type", "data": "skip"}
            ]
        });
        let blocks = extract_content(Some(&msg));
        assert_eq!(blocks.len(), 1);
    }

    // ── NoteStore 序列化/反序列化 ──

    #[test]
    fn note_store_roundtrip() {
        let mut notes = std::collections::HashMap::new();
        notes.insert("sess-1".to_string(), "第一个备注".to_string());
        notes.insert("sess-2".to_string(), "second note".to_string());
        let store = NoteStore { notes };

        let json = serde_json::to_string(&store).unwrap();
        let restored: NoteStore = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.notes.len(), 2);
        assert_eq!(restored.notes["sess-1"], "第一个备注");
        assert_eq!(restored.notes["sess-2"], "second note");
    }

    #[test]
    fn note_store_empty() {
        let store = NoteStore {
            notes: std::collections::HashMap::new(),
        };
        let json = serde_json::to_string(&store).unwrap();
        assert_eq!(json, r#"{"notes":{}}"#);
    }

    // ── MessageRole 序列化 ──

    #[test]
    fn message_role_serde() {
        assert_eq!(serde_json::to_string(&MessageRole::User).unwrap(), r#""user""#);
        assert_eq!(
            serde_json::to_string(&MessageRole::Assistant).unwrap(),
            r#""assistant""#
        );
        assert_eq!(serde_json::to_string(&MessageRole::System).unwrap(), r#""system""#);
    }

    #[test]
    fn message_role_deserialize() {
        let u: MessageRole = serde_json::from_str(r#""user""#).unwrap();
        assert_eq!(u, MessageRole::User);
        let a: MessageRole = serde_json::from_str(r#""assistant""#).unwrap();
        assert_eq!(a, MessageRole::Assistant);
    }

    // ── Provider 序列化/反序列化 ──

    #[test]
    fn provider_roundtrip() {
        let provider = Provider {
            id: "test-id".to_string(),
            name: "Test Provider".to_string(),
            notes: Some("备注".to_string()),
            website: Some("https://example.com".to_string()),
            api_key: "sk-test".to_string(),
            base_url: "https://api.test.com".to_string(),
            main_model: "sonnet".to_string(),
            opus_model: "model-opus".to_string(),
            sonnet_model: "model-sonnet".to_string(),
            haiku_model: "model-haiku".to_string(),
            sub_agent_model: "haiku".to_string(),
            reasoning_level: "max".to_string(),
            is_active: true,
        };

        let json = serde_json::to_string(&provider).unwrap();
        let restored: Provider = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.id, "test-id");
        assert_eq!(restored.name, "Test Provider");
        assert_eq!(restored.notes, Some("备注".to_string()));
        assert_eq!(restored.api_key, "sk-test");
        assert!(restored.is_active);
    }

    #[test]
    fn provider_optional_fields_none() {
        let provider = Provider {
            id: "id".to_string(),
            name: "Name".to_string(),
            notes: None,
            website: None,
            api_key: String::new(),
            base_url: String::new(),
            main_model: String::new(),
            opus_model: String::new(),
            sonnet_model: String::new(),
            haiku_model: String::new(),
            sub_agent_model: String::new(),
            reasoning_level: String::new(),
            is_active: false,
        };

        let json = serde_json::to_string(&provider).unwrap();
        assert!(!json.contains("notes"));
        assert!(!json.contains("website"));

        let restored: Provider = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.notes, None);
        assert_eq!(restored.website, None);
    }

    #[test]
    fn provider_store_roundtrip() {
        let store = ProviderStore {
            providers: vec![Provider {
                id: "1".to_string(),
                name: "P1".to_string(),
                notes: None,
                website: None,
                api_key: String::new(),
                base_url: String::new(),
                main_model: String::new(),
                opus_model: String::new(),
                sonnet_model: String::new(),
                haiku_model: String::new(),
                sub_agent_model: String::new(),
                reasoning_level: String::new(),
                is_active: true,
            }],
        };

        let json = serde_json::to_string(&store).unwrap();
        let restored: ProviderStore = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.providers.len(), 1);
        assert_eq!(restored.providers[0].name, "P1");
    }

    // ── derive_anthropic_model ──

    #[test]
    fn derive_model_haiku() {
        assert_eq!(
            derive_anthropic_model("haiku", "model-h", "model-s", "model-o"),
            "model-h"
        );
    }

    #[test]
    fn derive_model_sonnet() {
        assert_eq!(
            derive_anthropic_model("sonnet", "model-h", "model-s", "model-o"),
            "model-s"
        );
    }

    #[test]
    fn derive_model_opus() {
        assert_eq!(
            derive_anthropic_model("opus", "model-h", "model-s", "model-o"),
            "model-o"
        );
    }

    #[test]
    fn derive_model_fallback() {
        assert_eq!(
            derive_anthropic_model("custom-id", "model-h", "model-s", "model-o"),
            "custom-id"
        );
    }
}
