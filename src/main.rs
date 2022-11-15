mod api;
mod app;
mod commands;
mod domain;
mod entities;

use crate::api::messages::messages_filter;
use crate::api::websocket::websocket_filter;
use crate::domain::chat::Chat;
use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use migration::Migrator;
use settings::Settings;
use std::io::Write;
use tokio::select;
use warp::{serve, Filter};

#[tokio::main]
async fn main() {
    let settings = Settings::new().expect("Ошибка при загрузке конфига");

    let db = Migrator::set_up_db(
        &settings.database.host,
        &settings.database.login,
        &settings.database.password,
        &settings.database.name,
        false,
    )
    .await
    .expect("Ошибка подключения к базе данных");

    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    let chat = Chat::new(db.clone());

    let (chat_task, chat_connector) = chat.start();
    let server = serve(messages_filter(db.clone()).or(websocket_filter(chat_connector)));

    let server_task = server.run(([0, 0, 0, 0], settings.server.port));

    select! {
        _ = chat_task => {
            log::info!("Чат остановлен")
        }
         _ = server_task => {
            log::info!("Сервер остановлен")
        }
    }
}
