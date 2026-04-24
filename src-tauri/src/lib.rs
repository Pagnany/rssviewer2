use serde::{Deserialize, Serialize};
use tauri_plugin_http::reqwest;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Wayland fix
    std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");

    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, test_http])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RssFeed {
    pub id: String,
    pub feed_name: String,
    pub header: String,
    pub description: String,
    pub url: String,
    pub image: String,
    pub date: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RssFeedChannel {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub active: bool,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn test_http(url: &str) -> Result<(), String> {
    let res = reqwest::get(url).await.map_err(|e| e.to_string())?;
    let body = res.text().await.map_err(|e| e.to_string())?;
    println!("Response: {}", body);
    Ok(())
}
