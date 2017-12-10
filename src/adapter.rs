
use CQPAPP;
use types::{ Event, Request, LogPriority, Chat, Identity, Error };

pub fn initialize() -> i32 {
    0
}

pub fn startup() -> i32 {
    0
}

pub fn exit() -> i32 {
    0
}

pub fn enable() -> i32 {
    0
}

pub fn disable() -> i32 {
    0
}

pub fn message(chat: Chat, ident: Identity, message: &str) -> i32 {
    unsafe {
        CQPAPP.send_message(chat, message);
    }
    Event::Block as i32
}
