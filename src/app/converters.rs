use crate::domain::chat_room::{ChatMessage, MessageId};
use crate::entities::messages;

impl From<messages::Model> for ChatMessage {
    fn from(entity: messages::Model) -> Self {
        ChatMessage {
            id: MessageId(entity.id),
            user: serde_json::from_value(entity.user).unwrap(),
            to: entity.to.map(|e| serde_json::from_value(e).unwrap()),
            date_time: entity.date_time.into(),
            text: entity.text,
        }
    }
}
