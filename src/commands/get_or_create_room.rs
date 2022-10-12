use crate::{
    app::cqrs::{AppResult, Command},
    domain::chat_room::{ChatMessage, ChatRoomId},
    entities::prelude::*,
    entities::{self, messages},
};

use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait, QueryOrder, QuerySelect,
};

pub struct Context<'a> {
    pub db: &'a DatabaseConnection,
    pub room_id: ChatRoomId,
    pub room_name: &'a str,
    pub limit: u64,
}

#[async_trait]
impl Command for Context<'_> {
    type Out = Vec<ChatMessage>;

    async fn execute(&'_ self) -> AppResult<Vec<ChatMessage>> {
        let data = Rooms::find_by_id(self.room_id.0)
            .find_with_related(Messages)
            .order_by_desc(messages::Column::DateTime)
            .limit(self.limit)
            .all(self.db)
            .await?;

        if !data.is_empty() {
            Ok(data[0].1.iter().map(|m| m.to_owned().into()).collect())
        } else {
            let room = entities::rooms::ActiveModel {
                id: ActiveValue::Set(self.room_id.0),
                name: ActiveValue::Set(self.room_name.to_owned()),
            };

            room.insert(self.db).await?;

            Ok(vec![])
        }
    }
}
