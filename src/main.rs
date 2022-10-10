pub mod api;
pub mod domain;
pub mod settings;

use warp::{serve};
use tokio::select;
use crate::api::websocket::{websocket_filter};
use crate::domain::chat::{Chat};
use crate::settings::{Settings};
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;
use chrono::Local;
use migration::{Migrator};

#[tokio::main]
async fn main() {

    let settings = Settings::new().expect("Ошибка при загрузке конфига");
    
    Migrator::set_up_db("postgres", "postgres", "localhost", "chat", false).await.expect("Ошибка подключения к базе данных");

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
        .run(([0,0,0,0], settings.server.port));

    select! {
        _ = chat_task => {
            log::info!("Чат остановлен")
        }
         _ = server_task => {
            log::info!("Сервер остановлен")
        }
    }
}

