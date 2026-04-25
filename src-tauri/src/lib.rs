use chrono::{DateTime, Local, Utc};
use futures::{stream, SinkExt, StreamExt};
use regex::Regex;
use serde::{Deserialize, Serialize};
use tauri_plugin_http::reqwest;
use tauri_plugin_http::reqwest::header::USER_AGENT;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Wayland fix
    std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");

    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_rssfeeds])
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
    pub active: i32,
}

#[tauri::command]
async fn get_rssfeeds() -> Result<String, String> {
    // Only active channels
    let channels = get_rssfeed_channels()
        .await?
        .into_iter()
        .filter(|c| c.active == 1)
        .collect::<Vec<_>>();

    // Concurrent fetching of RSS feed channels
    let client = reqwest::Client::new();
    let concurrency = 10;

    let results: Vec<Result<String, String>> = stream::iter(channels)
        .map(|channel| {
            let client = client.clone();
            async move {
                let res = client
                    .get(&channel.url)
                    .header(USER_AGENT, "Rssviewer2/0.0.0")
                    .send()
                    .await
                    .map_err(|e| e.to_string())?;

                let body = res.text().await.map_err(|e| e.to_string())?;
                println!("Fetched {}: {} bytes", channel.name, body.len());
                // return body
                Ok(body)
            }
        })
        .buffer_unordered(concurrency)
        .collect()
        .await;

    // Filter and parse
    let rssfeeditems: Vec<RssFeed> = results
        .into_iter()
        .filter_map(|res| match res {
            Ok(feed) => Some(get_items_form_feed(&feed)),
            Err(e) => {
                eprintln!("Error fetching feed: {}", e);
                None
            }
        })
        .flatten()
        .collect();

    for item in rssfeeditems.iter().clone() {
        println!("{} - {}", item.header, item.url);
    }

    Ok(format!("{:?}", rssfeeditems))
}

async fn get_rssfeed_channels() -> Result<Vec<RssFeedChannel>, String> {
    let url = "https://www.pagnany.de/api/rss.php";
    let params = [("action", "get_channels")];
    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .form(&params)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let body = res.text().await.map_err(|e| e.to_string())?;
    let channels: Vec<RssFeedChannel> = serde_json::from_str(&body).map_err(|e| e.to_string())?;
    Ok(channels)
}

fn get_items_form_feed(feed: &str) -> Vec<RssFeed> {
    let mut rss_feed_vec = Vec::new();

    let doc = roxmltree::Document::parse(feed).unwrap();
    let mut rss_feed_name = String::from("");
    let mut rss_feed_name_set = false;
    for node in doc.descendants() {
        if node.tag_name().name() == "item" {
            let mut rss_feed: RssFeed = Default::default();

            for child in node.children() {
                match child.tag_name().name() {
                    "guid" => rss_feed.id = child.text().unwrap().to_string(),
                    "title" => rss_feed.header = child.text().unwrap().to_string(),
                    "description" => rss_feed.description = child.text().unwrap().to_string(),
                    "link" => rss_feed.url = child.text().unwrap().to_string(),
                    "pubDate" => {
                        let utc_dt: DateTime<Utc> =
                            DateTime::parse_from_rfc2822(child.text().unwrap())
                                .unwrap()
                                .with_timezone(&Utc);
                        let date = utc_dt.with_timezone(&Local);
                        rss_feed.date = date.format("%Y-%m-%d %H:%M").to_string();
                    }
                    "enclosure" => {
                        if child.attribute("type").unwrap() == "image/jpeg" {
                            rss_feed.image = child.attribute("url").unwrap().to_string();
                        }
                    }
                    "encoded" => {
                        // find <img> tag in a non xml string and get the link in src=""
                        let content_encoded = child.text().unwrap().to_string();
                        let re = Regex::new(r#"<img src="([^"]*)""#).unwrap();
                        let caps = re.captures(&content_encoded);

                        if let Some(caps) = caps {
                            let img_src = &caps[1];
                            rss_feed.image = img_src.to_string();
                        }
                    }
                    _ => (),
                }
            }
            rss_feed.feed_name = rss_feed_name.clone();
            rss_feed_vec.push(rss_feed);
        } else if node.tag_name().name() == "title" && !rss_feed_name_set {
            rss_feed_name = node.text().unwrap().to_string();
            rss_feed_name_set = true;
        }
    }

    rss_feed_vec
}
