use flutter_rust_bridge::StreamSink;
use flutter_rust_bridge::support::lazy_static;
use tokio::runtime::Runtime;
use crate::{live};
use crate::model::ShowType::Error;

pub fn say_hello() -> String {
    return String::from("HELLO");
}

pub fn get_room_stream(platform: String, rid: String) -> String {
    let rt = Runtime::new().unwrap();
    let ret = rt.block_on(async {
        match platform.as_str() {
            "bilibili" => live::bili::get(rid.as_str()).await,
            "douyu" => live::douyu::get(rid.as_str()).await,
            "huya" => live::huya::get(rid.as_str()).await,
            "cc" => live::cc::get(rid.as_str()).await,
            _ => Ok(Error(String::from("not impl")))
        }
    });
    return ret.unwrap().to_string();
}

pub fn get_room_danmaku(sink: StreamSink<String>, platform: String, rid: String) {}