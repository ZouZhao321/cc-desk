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
