use crate::discovery::message::MessageType;

pub const ERR: MessageType = MessageType{code: 255,repr: "ERR"};
pub const ANS: MessageType = MessageType{code: 0,repr: "ANS"};
pub const AUTH_OK: MessageType = MessageType{code: 1,repr: "AUTH_OK"};
pub const AUTH_FAILED: MessageType = MessageType{code: 128,repr: "AUTH_FAILED"};
