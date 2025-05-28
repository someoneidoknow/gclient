use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use dirs::config_dir;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn save_credentials(
    username: String,
    password: String,
    cf_clearance: Option<String>,
) -> Result<(), String> {
    use keyring::Entry;
    let username_entry = Entry::new("TwoBlade", "username")
        .map_err(|e| format!("Failed to create username entry: {}", e))?;
    username_entry.set_password(&username)
        .map_err(|e| format!("Failed to save username: {}", e))?;
    let password_entry = Entry::new("TwoBlade", &format!("password_{}", username))
        .map_err(|e| format!("Failed to create password entry: {}", e))?;
    password_entry.set_password(&password)
        .map_err(|e| format!("Failed to save password: {}", e))?;
    if let Some(cf) = cf_clearance {
        let cf_entry = Entry::new("TwoBlade", &format!("cf_clearance_{}", username))
            .map_err(|e| format!("Failed to create CF clearance entry: {}", e))?;
        cf_entry.set_password(&cf)
            .map_err(|e| format!("Failed to save CF clearance: {}", e))?;
    }
    Ok(())
}

#[tauri::command]
async fn load_credentials() -> Result<HashMap<String, String>, String> {
    use keyring::Entry;
    let username_entry = Entry::new("TwoBlade", "username")
        .map_err(|e| format!("Failed to create username entry: {}", e))?;
    let username = username_entry.get_password()
        .map_err(|e| format!("No saved credentials found: {}", e))?;
    let password_entry = Entry::new("TwoBlade", &format!("password_{}", username))
        .map_err(|e| format!("Failed to create password entry: {}", e))?;
    let password = password_entry.get_password()
        .map_err(|e| format!("Failed to load password: {}", e))?;
    let cf_clearance = {
        let cf_entry = Entry::new("TwoBlade", &format!("cf_clearance_{}", username))
            .map_err(|e| format!("Failed to create CF clearance entry: {}", e)).ok();
        if let Some(entry) = cf_entry {
            entry.get_password().ok()
        } else {
            None
        }
    };
    let mut result = HashMap::new();
    result.insert("username".to_string(), username);
    result.insert("password".to_string(), password);
    if let Some(cf) = cf_clearance {
        result.insert("cf_clearance".to_string(), cf);
    }
    Ok(result)
}

#[tauri::command]
async fn delete_credentials(username: String) -> Result<(), String> {
    use keyring::Entry;
    if let Ok(username_entry) = Entry::new("TwoBlade", "username") {
        let _ = username_entry.delete_credential();
    }
    if let Ok(password_entry) = Entry::new("TwoBlade", &format!("password_{}", username)) {
        let _ = password_entry.delete_credential();
    }
    if let Ok(cf_entry) = Entry::new("TwoBlade", &format!("cf_clearance_{}", username)) {
        let _ = cf_entry.delete_credential();
    }
    Ok(())
}

#[tauri::command]
async fn login_twoblade_and_save(
    username: String,
    password: String,
    cf_clearance: Option<String>,
    save_credentials_flag: bool,
) -> Result<HashMap<String, String>, String> {
    let result = login_twoblade(username.clone(), password.clone(), cf_clearance.clone()).await?;
    if save_credentials_flag {
        save_credentials(username, password, cf_clearance).await?;
    }
    Ok(result)
}

#[tauri::command]
async fn login_twoblade(
    username: String,
    password: String,
    cf_clearance: Option<String>,
) -> Result<HashMap<String, String>, String> {
    use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
    use reqwest::{Client, Url};
    use reqwest::cookie::{Jar, CookieStore};
    use std::sync::Arc;
    let base_url = "https://twoblade.com";
    let login_url = format!("{}/login", base_url);
    let jar = Arc::new(Jar::default());
    let url = Url::parse(base_url).unwrap();
    if let Some(cf) = &cf_clearance {
        jar.add_cookie_str(&format!("cf_clearance={}", cf), &url);
    }
    let client = Client::builder()
        .cookie_provider(jar.clone())
        .build()
        .map_err(|e| format!("client build: {}", e))?;
    let mut headers = HeaderMap::new();
    headers.insert("accept", HeaderValue::from_static("application/json"));
    headers.insert("accept-language", HeaderValue::from_static("en-US,en;q=0.9,ro;q=0.8"));
    headers.insert("cache-control", HeaderValue::from_static("no-cache"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"));
    headers.insert("pragma", HeaderValue::from_static("no-cache"));
    headers.insert("priority", HeaderValue::from_static("u=1, i"));
    headers.insert("sec-ch-ua", HeaderValue::from_static("\"Chromium\";v=\"136\", \"Google Chrome\";v=\"136\", \"Not.A/Brand\";v=\"99\""));
    headers.insert("sec-ch-ua-arch", HeaderValue::from_static("\"x86\""));
    headers.insert("sec-ch-ua-bitness", HeaderValue::from_static("\"64\""));
    headers.insert("sec-ch-ua-full-version", HeaderValue::from_static("\"136.0.7103.114\""));
    headers.insert("sec-ch-ua-full-version-list", HeaderValue::from_static("\"Chromium\";v=\"136.0.7103.114\", \"Google Chrome\";v=\"136.0.7103.114\", \"Not.A/Brand\";v=\"99.0.0.0\""));
    headers.insert("sec-ch-ua-mobile", HeaderValue::from_static("?0"));
    headers.insert("sec-ch-ua-model", HeaderValue::from_static("\"\""));
    headers.insert("sec-ch-ua-platform", HeaderValue::from_static("\"Windows\""));
    headers.insert("sec-ch-ua-platform-version", HeaderValue::from_static("\"10.0.0\""));
    headers.insert("sec-fetch-dest", HeaderValue::from_static("empty"));
    headers.insert("sec-fetch-mode", HeaderValue::from_static("cors"));
    headers.insert("sec-fetch-site", HeaderValue::from_static("same-origin"));
    headers.insert("x-sveltekit-action", HeaderValue::from_static("true"));
    headers.insert("referer", HeaderValue::from_static("https://twoblade.com/login"));
    headers.insert("origin", HeaderValue::from_static("https://twoblade.com"));
    let payload = format!("username={}&password={}", username, password);
    let resp = client
        .post(&login_url)
        .headers(headers)
        .body(payload)
        .send()
        .await
        .map_err(|e| format!("request error: {e}"))?;
    let status = resp.status();
    let body = resp.text().await.unwrap_or_default();
    if !status.is_success() {
        return Err(format!("Login failed: {} — {}", status, body));
    }
    let cookies = jar
        .cookies(&url)
        .map(|c| c.to_str().unwrap_or("").to_string())
        .unwrap_or_default();
    let mut out = HashMap::new();
    for pair in cookies.split(';') {
        let mut parts = pair.trim().splitn(2, '=');
        if let (Some(k), Some(v)) = (parts.next(), parts.next()) {
            out.insert(k.to_string(), v.to_string());
        }
    }
    Ok(out)
}

#[derive(Serialize, Deserialize)]
pub struct AppSettings {
    pub anti_spam_mode: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        AppSettings {
            anti_spam_mode: "smart".to_string(),
        }
    }
}

fn settings_path() -> Result<PathBuf, String> {
    config_dir()
        .ok_or_else(|| "No config dir found".to_string())
        .map(|mut dir| {
            dir.push("2bclient");
            dir.push("settings.json");
            dir
        })
}

#[tauri::command]
async fn save_settings(settings: AppSettings) -> Result<(), String> {
    let path = settings_path()?;
    let json = serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?;
    fs::create_dir_all(path.parent().unwrap()).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn load_settings() -> Result<AppSettings, String> {
    let path = settings_path()?;
    if !path.exists() {
        return Ok(AppSettings::default());
    }
    let data = fs::read_to_string(path).map_err(|e| e.to_string())?;
    serde_json::from_str(&data).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(tauri::generate_handler![
            greet, 
            login_twoblade, 
            login_twoblade_and_save,
            save_credentials, 
            load_credentials, 
            delete_credentials,
            save_settings,
            load_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
