use crate::bindings::ChatModule_ChatMessage;
use std::pin::Pin;

pub struct ChatModuleChatMessage {
    pub inner: Option<Pin<Box<ChatModule_ChatMessage>>>,
}
