// This is the entry point of your Rust library.
// When adding new code to your project, note that only items used
// here will be transformed to their Dart equivalents.

use std::collections::HashMap;
use anyhow::anyhow;
use flutter_rust_bridge::StreamSink;
use tokio::runtime::Runtime;
use seam_core::live;

pub fn get_stream(platform: String, rid: String, headers: Option<Vec<String>>) -> anyhow::Result<String> {
    let node = match live::all().get(&platform) {
        None => {
            return Err(anyhow!("platform not support: {}", platform));
        }
        Some(client) => {
            let mut request_headers: HashMap<String, String> = HashMap::new();
            if headers.is_some() {
                for chunk in headers.unwrap().chunks(2) {
                    if chunk.len() == 2 {
                        request_headers.insert(chunk[0].clone(), chunk[1].clone());
                    }
                }
            }
            let rt = Runtime::new().unwrap();
            rt.block_on(client.get(&rid, Some(request_headers)))
        }
    };

    return match node {
        Ok(node) => {
            anyhow::Ok(node.json())
        }
        Err(e) => {
            Err(anyhow!("error: {}", e))
        }
    };
}

pub fn start_fetch_danmaku(sink: StreamSink<i8>, platform: String, rid: String) {}