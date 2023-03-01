mod common;
mod config;
mod danmu;
mod live;
mod model;
mod recorder;
mod util;
mod api;
mod bridge_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */


// 为没有实现弹幕功能的直播平台添加默认空白实现
#[macro_export]
macro_rules! default_danmu_client {
    ($name: ident) => {
        use paste::paste;

        paste! {
            use async_trait::async_trait;
            use $crate::danmu::{Danmu, DanmuRecorder};

            pub struct [<$name DanmuClient>] {}

            impl [<$name DanmuClient>] {
                pub async fn try_new(_room_id: &str) -> Result<Self> {
                    Ok(Self {})
                }
            }

            #[async_trait]
            impl Danmu for [<$name DanmuClient>] {
                async fn start(&mut self, _recorder: Vec<&dyn DanmuRecorder>) -> Result<()> {
                    println!("该直播平台暂未实现弹幕功能。");
                    Ok(())
                }
            }
        }
    };
}