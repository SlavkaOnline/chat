use log;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::{broadcast, mpsc};
use tokio::task::JoinHandle;
use serde::{Deserialize, Serialize};

use super::chat_room::{ChatRoom, ChatRoomRef, ChatRoomId, ChatMessage};


#[derive(Eq, Hash, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct ConnectionId (pub uuid::Uuid);

enum ChatCommand {
    Connect {
        id: ChatRoomId,
        connection_id: ConnectionId,
        channel_in: broadcast::Sender<ChatMessage>,
        channel_out: mpsc::Sender<ChatMessage>,
    },
    Disconect {
        id: ChatRoomId,
        connection_id: ConnectionId
    },
    RemoveRoom {
        id: ChatRoomId
    }
}

#[derive(Clone)]
pub struct ChatRef {
    command_channel: Sender<ChatCommand>,
}

impl ChatRef {
    pub async fn connect_to_room(
        &self,
        id: ChatRoomId,
        connection_id: ConnectionId,
        channel_in: broadcast::Sender<ChatMessage>,
        channel_out: mpsc::Sender<ChatMessage>,
    ) {
        let connect_message = ChatCommand::Connect {
            id,
            connection_id,
            channel_in,
            channel_out,
        };

        self.command_channel.send(connect_message).await;
    }

    pub async fn disconect_from_room(
        &self, 
        id: ChatRoomId,
        connection_id: ConnectionId
    ) {
        let disconnect_message = ChatCommand::Disconect { id, connection_id };
        self.command_channel.send(disconnect_message).await;
    }

    pub async fn remove_room(
        &self, 
        id: ChatRoomId
    ) {
        let remove_room_command = ChatCommand::RemoveRoom {id};
        self.command_channel.send(remove_room_command).await;
    }

}

pub struct Chat {
    pub rooms: RefCell<HashMap<ChatRoomId, Arc<ChatRoomRef>>>,
}

impl Chat {
    pub fn new() -> Chat {
        let rooms = RefCell::new(HashMap::new());
        Chat { rooms }
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
                    ChatCommand::Connect {
                        id,
                        connection_id,
                        channel_in,
                        channel_out,
                    } => {
                        let chat_room_connector = self
                            .rooms
                            .borrow_mut()
                            .entry(id)
                            .or_insert(Arc::new(ChatRoom::new(id, internal_chat_ref.clone()).start()))
                            .clone();

                        chat_room_connector.connect(connection_id, channel_out).await;
                        
                        tokio::spawn(async move {
                            let mut rx = channel_in.subscribe();
                            drop(channel_in);
                            
                            while let Ok(msg) = rx.recv().await {
                                chat_room_connector.add_message(msg).await;
                            }
                            log::info!("Соединение {:?} прекратило читать сообщения", connection_id.0);
                        });
                    },

                ChatCommand::Disconect { id, connection_id } => {
                    if let Some(room_ref) = self.rooms.borrow().get(&id) {
                        let rc =  room_ref.clone();
                        tokio::spawn(async move {
                            rc.disconnect(connection_id).await;
                        });                       
                    }
                }

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
