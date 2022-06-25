use std::convert::Infallible;
use chrono::Utc;
use futures::{SinkExt, StreamExt};
use crate::domain::chat::{ChatRef, ConnectionId};
use crate::domain::chat_room::{ChatMessage, ChatRoomId, MessageId, User};
use serde::{Deserialize};
use tokio::sync::{broadcast, mpsc};
use uuid::Uuid;
use warp::{Filter, Rejection, Reply, ws};
use warp::ws::WebSocket;


type Result<T> = std::result::Result<T, Rejection>;

#[derive(Clone, Deserialize)]
struct WsMessage {
    pub user: User,
    pub to: Option<User>,
    pub tags: Vec<String>,
    pub text: String,
}

pub fn websocket_filter(chat_ref: ChatRef) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("chats" / Uuid)
        .and(warp::ws())
        .and(with_chat_connector(chat_ref.clone()))
        .and_then(websocket_connect)
}

fn with_chat_connector(chat_ref: ChatRef) -> impl Filter<Extract = (ChatRef,), Error = Infallible> + Clone {
    warp::any().map(move || chat_ref.clone())
}

async fn websocket_connect(id: Uuid, ws: ws::Ws, chat_ref: ChatRef) -> Result<impl Reply> {
    let _chat_connector = chat_ref.clone();
    let ws_upgrade = ws.on_upgrade(move |web_socket| {
        connection_handle(id, web_socket, _chat_connector)
    });
    Ok(ws_upgrade)
}

async fn connection_handle(
    id: Uuid,
    web_socket: WebSocket,
    chat_connector: ChatRef,
) {
    let connection_id = ConnectionId(Uuid::new_v4());
    let (mut websoket_in, mut websoket_out) = web_socket.split();
   
    let (chat_room_channel, _) = broadcast::channel::<ChatMessage>(1);
    let (chat_room_in, mut chat_room_out) = mpsc::channel::<ChatMessage>(100);

    let room_id = ChatRoomId(id);
    chat_connector.connect_to_room(room_id, connection_id, chat_room_channel.clone(), chat_room_in.clone()).await;

    tokio::spawn(async move {
        while let Some(result) = websoket_out.next().await {
            match result {
                Ok(message) => 
                if let Ok(text) = message.to_str() {
                    if let Ok(msg) = serde_json::from_str::<WsMessage>(text) {
                        if let Err(err) = chat_room_channel.send(ChatMessage {
                            id: MessageId(Uuid::new_v4()),
                            date_time: Utc::now(),
                            user: msg.user,
                            to: msg.to,
                            tags: msg.tags,
                            text: msg.text,
                        }) {
                            log::error!("Ошибка при отправке сообщения в комнату {}, {}", id, err.to_string())
                        }
                    } else {
                        log::error!("Ошибка при парсинге сообщения");
                    }
                },
                Err(e) => eprintln!("Ошибка при чтении из вебсокета {:?}", e)
            }
        }
        drop(chat_room_channel);
        drop(chat_room_in);
        chat_connector.disconect_from_room(room_id, connection_id).await;
    });

    tokio::spawn(async move {
        while let Some(msg) = chat_room_out.recv().await {
            if let Err(err) = websoket_in.send(ws::Message::text(serde_json::to_string(&msg).unwrap())).await {
                log::error!("Ошибка при отправке сообщения в вебсокет {}", err.to_string());
                break;
            }
        }
    });
}