use std::pin::Pin;
use crate::bindings::ChatModule_ChatMessage;

pub struct ChatModuleChatMessage {
    pub inner: Option<Pin<Box<ChatModule_ChatMessage>>>,
}