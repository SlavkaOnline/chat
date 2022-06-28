pub mod chat_room {
    use std::cell::{RefCell, Ref};
    use chrono::{DateTime, Utc};

    pub struct UserId {
        pub value: uuid::Uuid,
    }

    pub struct MessageId {
        pub value: uuid::Uuid,
    }

    #[derive(Eq, Hash, PartialEq, Copy, Clone)]
    pub struct ChatRoomId {
        pub value: uuid::Uuid,
    }

    pub struct User {
        pub id: UserId,
        pub name: String,
    }

    pub struct Message {
        pub id: MessageId,
        pub date_time: DateTime<Utc>,
        pub user: User,
        pub to: Option<Box<User>>,
        pub tags: Vec<String>,
        pub text: String,
    }

    pub struct ChatRoom {
        pub id: ChatRoomId,
        messages: RefCell<Vec<Message>>,
    }



    impl ChatRoom {

        pub fn new(id: ChatRoomId) -> ChatRoom {
            return ChatRoom { id, messages: RefCell::new(Vec::new()) };
        }

        pub fn add_message(&self, message: Message) {
            self.messages.borrow_mut().push(message)
        }

        pub fn get_messages(&self) -> Ref<Vec<Message>> {
            return self.messages.borrow();
        }

    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use uuid::Uuid;
        use chrono::{Utc};

        #[test]
        fn message_add() {
            let room = ChatRoom::new(ChatRoomId { value: Uuid::new_v4() });

            let user = User { id: UserId { value: Uuid::new_v4() }, name: String::from("user") };
            let message = Message {
                id: MessageId { value: Uuid::new_v4() },
                date_time: Utc::now(),
                user: user,
                to: None,
                tags: vec![String::from("system")],
                text: String::from("message"),
            };

            room.add_message(message);

            let count = Vec::len(&room.get_messages());

            assert_eq!(1, count);
        }
    }


}

