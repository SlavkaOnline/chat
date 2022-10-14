use log;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::{broadcast, mpsc, oneshot};
use tokio::task::JoinHandle;

use crate::app::cqrs::Command;
use crate::commands;

use super::chat_room::{ChatMessage, ChatRoom, ChatRoomId, ChatRoomRef};

#[derive(Eq, Hash, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct ConnectionId(pub uuid::Uuid);

enum ChatCommand {
    GetRoom {
        id: ChatRoomId,
        resp: oneshot::Sender::<Arc<ChatRoomRef>>
    },
    RemoveRoom {
        id: ChatRoomId,
    },
}

#[derive(Clone)]
pub struct ChatRef {
    command_channel: Sender<ChatCommand>,
}

impl ChatRef {
    pub async fn get_room(
        &self,
        id: ChatRoomId
        ) -> Arc<ChatRoomRef> {
        let (sender, receiver) = oneshot::channel::<Arc<ChatRoomRef>>();
        self.command_channel.send(ChatCommand::GetRoom {
            id,
            resp: sender
        }).await;
        let chat_room_ref = receiver.await;
        chat_room_ref.unwrap()
    }

    pub async fn remove_room(&self, id: ChatRoomId) {
        let remove_room_command = ChatCommand::RemoveRoom { id };
        self.command_channel.send(remove_room_command).await;
    }
}

pub struct Chat {
    pub rooms: RefCell<HashMap<ChatRoomId, Arc<ChatRoomRef>>>,
    db: DatabaseConnection,
}

impl Chat {
    pub fn new(db: DatabaseConnection) -> Chat {
        let rooms = RefCell::new(HashMap::new());
        Chat { rooms, db }
    }

    pub fn start(self) -> (JoinHandle<()>, ChatRef) {
        let (sender, mut receiver) = mpsc::channel::<ChatCommand>(10);

        let chat_ref = ChatRef {
            command_channel: sender.clone(),
        };

        let internal_chat_ref = chat_ref.clone();

        let task = tokio::spawn(async move {
            while let Some(message) = receiver.recv().await {
                match message {
                    ChatCommand::GetRoom { id, resp }
                     => {
                        let chat_room_connector = self
                            .rooms
                            .borrow_mut()
                            .entry(id)
                        .or_insert(Arc::new(
                                ChatRoom::new(id, internal_chat_ref.clone(), self.db.clone())
                                .start(),
                        )).clone();
                        resp.send(chat_room_connector);
                    },

                    ChatCommand::RemoveRoom { id } => {
                        self.rooms.borrow_mut().remove(&id);

                        log::info!("Комната {} удалена", id.0);
                    }
                }
            }
            log::info!("Чат перестал считывать сообщения");
        });

        (task, chat_ref)
    }
}
