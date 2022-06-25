pub mod api;
pub mod domain;

use warp::{serve};
use tokio::select;
use crate::api::websocket::{websocket_filter};
use crate::domain::chat::{Chat};
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;
use chrono::Local;

#[tokio::main]
async fn main() {

    Builder::new()
        .format(|buf, record| {
            writeln!(buf,
                     "{} [{}] - {}",
                     Local::now().format("%Y-%m-%dT%H:%M:%S"),
                     record.level(),
                     record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    let chat = Chat::new();

    let (chat_task, chat_connector) = chat.start();
    let server = serve(websocket_filter(chat_connector));

    let server_task = server
        .run(([0,0,0,0], 80));

    select! {
        _ = chat_task => {
            log::info!("Чат остановлен")
        }
         _ = server_task => {
            log::info!("Сервер остановлен")
        }
    }
}

