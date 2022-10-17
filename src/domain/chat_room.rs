use crate::app::cqrs::Command;
use crate::commands;

use super::chat::{ChatRef, ConnectionId};
use super::circle_buffer::CircleBuffer;
use chrono::{DateTime, Utc};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;

#[derive(Eq, Hash, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct UserId(pub uuid::Uuid);

#[derive(Eq, Hash, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct MessageId(pub uuid::Uuid);

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct ChatRoomId(pub uuid::Uuid);

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub name: String,
}

pub struct Connection {
    id: ConnectionId,
    message_sender: mpsc::Sender<ChatMessage>,
}

#[derive(Clone, Serialize)]
pub struct ChatMessage {
    pub id: MessageId,
    pub date_time: DateTime<Utc>,
    pub user: User,
    pub to: Option<User>,
    pub text: String,
}

enum ChatRoomCommand {
    AddMessage(ChatMessage),
    Connect {
        connection_id: ConnectionId,
        message_sender: mpsc::Sender<ChatMessage>,
    },
    Disconnect {
        connection_id: ConnectionId,
    },
    RemoveRoom,
}

#[derive(Clone)]
pub struct ChatRoomRef {
    command_channel: mpsc::Sender<ChatRoomCommand>,
}

impl ChatRoomRef {
    pub async fn add_message(&self, message: ChatMessage) {
        self.command_channel
            .send(ChatRoomCommand::AddMessage(message))
            .await;
    }

    pub async fn connect(
        &self,
        connection_id: ConnectionId,
        message_sender: mpsc::Sender<ChatMessage>,
    ) {
        self.command_channel
            .send(ChatRoomCommand::Connect {
                connection_id,
                message_sender,
            })
            .await;
    }

    pub async fn disconnect(&self, connection_id: ConnectionId) {
        self.command_channel
            .send(ChatRoomCommand::Disconnect { connection_id })
            .await;
    }

    async fn remove_room(&self) {
        self.command_channel.send(ChatRoomCommand::RemoveRoom).await;
    }
}

pub struct ChatRoom {
    pub id: ChatRoomId,
    messages: CircleBuffer<ChatMessage>,
    connections: HashMap<ConnectionId, Connection>,
    chat: ChatRef,
    pendong_to_remove: bool,
    db: DatabaseConnection,
}

impl ChatRoom {
    pub fn new(id: ChatRoomId, chat: ChatRef, db: DatabaseConnection) -> ChatRoom {
        return ChatRoom {
            id,
            messages: CircleBuffer::with_capacity(10),
            connections: HashMap::with_capacity(10),
            chat,
            pendong_to_remove: false,
            db,
        };
    }

    pub fn with_messages(
        id: ChatRoomId,
        messages: Vec<ChatMessage>,
        chat: ChatRef,
        db: DatabaseConnection,
    ) -> ChatRoom {
        return ChatRoom {
            id,
            messages: CircleBuffer::from(messages),
            connections: HashMap::with_capacity(10),
            chat,
            pendong_to_remove: false,
            db,
        };
    }

    pub fn start(mut self) -> ChatRoomRef {
        let (sender, mut receiver) = mpsc::channel::<ChatRoomCommand>(10);

        let chat_room_ref = ChatRoomRef {
            command_channel: sender.clone(),
        };
        let internal_chat_room_ref = chat_room_ref.clone();
        tokio::spawn(async move {
            let result = commands::get_or_create_room::Context {
                room_id: self.id,
                room_name: "",
                db: &self.db,
                limit: 10,
            }
            .execute()
            .await;

            match result {
                Ok(messages) => self.messages = CircleBuffer::from(messages),
                Err(_) => self.chat.remove_room(self.id).await,
            }

            while let Some(msg) = receiver.recv().await {
                match msg {
                    ChatRoomCommand::AddMessage(message) => {
                        commands::add_message_to_room::Context {
                            db: &self.db,
                            message: message.clone(),
                            room_id: self.id,
                        }
                        .execute()
                        .await;

                        self.messages.push(message.clone());
                        for connection in &self.connections {
                            connection.1.message_sender.send(message.clone()).await;
                        }
                    }

                    ChatRoomCommand::Connect {
                        connection_id,
                        message_sender,
                    } => {
                        self.connections.insert(
                            connection_id,
                            Connection {
                                id: connection_id,
                                message_sender: message_sender.clone(),
                            },
                        );
                        for m in &self.messages {
                            message_sender.send(m.clone()).await;
                        }
                    }

                    ChatRoomCommand::Disconnect { connection_id } => {
                        self.connections.remove(&connection_id);
                        let _internal_chat_room_ref = internal_chat_room_ref.clone();
                        if self.connections.is_empty() && !self.pendong_to_remove {
                            self.pendong_to_remove = true;
                            tokio::spawn(async move {
                                sleep(Duration::from_secs(30)).await;
                                _internal_chat_room_ref.remove_room().await;
                            });
                        }
                    }

                    ChatRoomCommand::RemoveRoom => {
                        if self.connections.is_empty() {
                            self.chat.remove_room(self.id).await;
                        } else {
                            self.pendong_to_remove = false;
                        }
                    }
                }
            }
        });

        chat_room_ref
    }
}
