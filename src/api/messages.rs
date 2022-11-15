use crate::{entities::messages, entities::prelude::*};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use warp::{Filter, Rejection, Reply};

pub mod models {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    #[derive(Deserialize)]
    pub struct ChatMessagesRequest {
        pub room_id: Uuid,
    }

    #[derive(Serialize)]
    pub struct ChatMessagesResponse {
        pub room_id: Uuid,
        pub messages: Vec<ChatMessageApi>,
    }

    #[derive(Clone, Serialize, Deserialize)]
    pub struct UserApi {
        pub id: Uuid,
        pub name: String,
    }

    #[derive(Clone, Serialize)]
    pub struct ChatMessageApi {
        pub id: Uuid,
        pub date_time: DateTime<Utc>,
        pub user: UserApi,
        pub to: Option<UserApi>,
        pub text: String,
    }
}

pub fn messages_filter(
    db: DatabaseConnection,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("messages")
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::any().map(move || db.clone()))
        .and_then(messages_list)
}

async fn messages_list(
    req: models::ChatMessagesRequest,
    db: DatabaseConnection,
) -> Result<impl warp::Reply, warp::Rejection> {
    let result = Messages::find()
        .filter(messages::Column::RoomId.eq(req.room_id))
        .order_by_desc(messages::Column::DateTime)
        .limit(10)
        .all(&db)
        .await;

    let Ok(messages) = result else {
      return Err(warp::reject())
    };

    Ok(warp::reply::json(&models::ChatMessagesResponse {
        room_id: req.room_id,
        messages: messages.iter().map(|m| m.to_owned().into()).collect(),
    }))
}
