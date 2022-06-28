mod chat_room;
mod chat;

use std::rc::Rc;
use std::sync::{Arc, Mutex};
use chrono::Utc;
use futures::{FutureExt, StreamExt};
use uuid::Uuid;
use warp::{Filter, ws};
use crate::chat_room::chat_room::{ChatRoom, ChatRoomId, Message, MessageId, User, UserId};
use serde;
use warp::ws::WebSocket;
use crate::chat::chat::Chat;

#[tokio::main]
async fn main() {

    let chat = Arc::new(Mutex::new(Chat::new()));

    let ws_routes = warp::path!("chats" / Uuid)
        .and(warp::ws())
        .map(|id: Uuid, ws: ws::Ws| {
            let mut chat = chat.lock().unwrap();
            let room = chat.get_or_create_room(ChatRoomId{value: id});
            ws.on_upgrade(move |websocket| connection_handle(websocket, Rc::clone(room))
        });

    warp::serve(ws_routes)
        .run(([127, 0, 0, 1], 8080))
        .await;
}

fn connection_handle(websocket: WebSocket, room: Rc<ChatRoom>) {

    let (tx, mut rx) = websocket.split();

    tokio::spawn(async move  {
        while let Some(result) = rx.next().await {
            match result {
                Ok(message) =>
                    if let Ok(text) = message.to_str() {
                        room.add_message(Message {
                            id: MessageId { value: Uuid::new_v4() },
                            date_time: Utc::now(),
                            user: User { id: UserId { value: Uuid::new_v4() }, name: String::from("user") },
                            to: None,
                            tags: vec!["test".to_string()],
                            text: text.to_string()
                        })
                    }
                Err(e) =>
                    eprintln!("websocket error: {:?}", e)
            }
        }
    });
}
