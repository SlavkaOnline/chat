mod chat {

    use std::cell::{RefCell};
    use std::collections::{HashMap};
    use std::rc::Rc;
    use crate::chat_room::chat_room::{ChatRoom, ChatRoomId};

    struct Chat {
        rooms: RefCell<HashMap<ChatRoomId, Rc<ChatRoom>>>
    }

    impl Chat {
        pub fn new() -> Chat {
            Chat {rooms: RefCell::new(HashMap::new())}
        }

        pub fn get_or_create_room(&self, id: ChatRoomId) -> Rc<ChatRoom> {
           return Rc::clone(self.rooms.borrow_mut().entry(id).or_insert(Rc::new(ChatRoom::new(id))));
        }
    }
}