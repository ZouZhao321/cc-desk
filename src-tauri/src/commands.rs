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
    let derived_model_id = match config.model.as_str() {
        "haiku" => &config.haiku_id,
        "sonnet" => &config.sonnet_id,
        "opus" => &config.opus_id,
        _ => &config.model,
    };
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

fn env_str(env: &serde_json::Value, key: &str) -> String {
    env.get(key).and_then(|v| v.as_str()).unwrap_or_default().to_string()
}

fn json_str(json: &serde_json::Value, key: &str) -> String {
    json.get(key).and_then(|v| v.as_str()).unwrap_or_default().to_string()
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

    let entries = fs::read_dir(&projects_dir).map_err(|e| format!("读取 projects 目录失败: {e}"))?;

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
            let file = fs::File::open(&jsonl_path).map_err(|e| format!("打开 JSONL 文件失败: {e}"))?;
            let reader = BufReader::new(file);
            let mut message_count = 0;
            let mut meta = None;

            for line in reader.lines().map_while(Result::ok) {
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
