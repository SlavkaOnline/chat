pub mod chat_room {

    use std::cell::{RefCell, Ref};

    pub struct UserId {
        pub value: uuid::Uuid
    }

    pub struct MessageId {
        pub value: uuid::Uuid 
    }

    pub struct ChatRoomId {
        pub value: uuid::Uuid 
    }

    pub struct User {
        pub id: UserId,
        pub name: String
    }

    pub struct Message {
       pub id: MessageId,
       pub user: User,
       pub to: Option<Box<User>>,
       pub tags: Vec<String>,
       pub text: String
    }

    pub struct ChatRoom {
        pub id: ChatRoomId,
        users: RefCell<Vec<User>>,
        messages: RefCell<Vec<Message>>
    }

    pub fn create_chat(id: ChatRoomId) -> ChatRoom {
        return ChatRoom { id, users: RefCell::new(Vec::new()), messages: RefCell::new(Vec::new()) }        
    }

    impl ChatRoom {
        pub fn add_messsage(&self, message: Message) {
            self.messages.borrow_mut().push(message)
        }

        pub fn add_user(&self, user: User) {
            self.users.borrow_mut().push(user);
        }

        pub fn get_messages(&self) -> Ref<Vec<Message>> {
            return self.messages.borrow();
        }

        pub fn get_users(&self) -> Ref<Vec<User>> {
            return self.users.borrow();
        }
    }
    

}

#[cfg(test)]
    mod tests {
        
        use super::*;
        use uuid::Uuid;

        #[test]
        fn message_add() {
            let room = chat_room::create_chat(chat_room::ChatRoomId {value: Uuid::new_v4()});

            let user = chat_room::User {id: chat_room::UserId {value: Uuid::new_v4()}, name: String::from("user")};
            let message = chat_room::Message {
                id: chat_room::MessageId {value: Uuid::new_v4() },
                user: user,
                to: None,
                tags: vec![String::from("system")],
                text: String::from("message")
            };
    
            room.add_messsage(message);
    
            let count = Vec::len(&room.get_messages());
    
            assert_eq!(1, count);
        }

        #[test]
        fn user_add() {
            let room = chat_room::create_chat(chat_room::ChatRoomId {value: Uuid::new_v4()});
            let user = chat_room::User {id: chat_room::UserId {value: Uuid::new_v4()}, name: String::from("user")};

            room.add_user(user);

            let count = Vec::len(&room.get_users());
    
            assert_eq!(1, count);

        }
    }
