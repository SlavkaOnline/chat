use crate::{
    app::cqrs::{AppErr, AppResult, Command},
    domain::chat_room::{ChatMessage, ChatRoomId},
    entities,
};

use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection};

pub struct Context<'a> {
    pub db: &'a DatabaseConnection,
    pub message: ChatMessage,
    pub room_id: ChatRoomId,
}

#[async_trait]
impl Command for Context<'_> {
    type Out = ();

    async fn execute(&'_ self) -> AppResult<()> {
        let message = entities::messages::ActiveModel {
            id: ActiveValue::Set(self.message.id.0),
            user: ActiveValue::Set(serde_json::to_value(self.message.user.to_owned()).unwrap()),
            to: ActiveValue::Set(
                self.message
                    .to
                    .clone()
                    .map(|to| serde_json::to_value(to.to_owned()).unwrap()),
            ),
            date_time: ActiveValue::Set(self.message.date_time.into()),
            room_id: ActiveValue::Set(self.room_id.0),
            text: ActiveValue::Set(self.message.text.to_owned()),
        };

        if let Err(err) = message.insert(self.db).await {
            eprintln!("{:?}", err);
            return Err(AppErr::Bug(err.to_string()));
        }

        Ok(())
    }
}
