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

mod Api {
    use crate::{api::*, domain::chat_room, entities};

    impl From<chat_room::User> for messages::models::UserApi {
        fn from(model: chat_room::User) -> Self {
            messages::models::UserApi {
                id: model.id.0,
                name: model.name,
            }
        }
    }

    impl From<chat_room::ChatMessage> for messages::models::ChatMessageApi {
        fn from(model: chat_room::ChatMessage) -> Self {
            messages::models::ChatMessageApi {
                id: model.id.0,
                date_time: model.date_time,
                user: model.user.into(),
                to: model.to.map(|t| t.into()),
                text: model.text,
            }
        }
    }

    impl From<entities::messages::Model> for messages::models::ChatMessageApi {
        fn from(entity: entities::messages::Model) -> Self {
            messages::models::ChatMessageApi {
                id: entity.id,
                user: serde_json::from_value(entity.user).unwrap(),
                to: entity.to.map(|e| serde_json::from_value(e).unwrap()),
                date_time: entity.date_time.into(),
                text: entity.text,
            }
        }
    }
}
