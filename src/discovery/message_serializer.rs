use crate::discovery::message::Message;
use crate::discovery::ImplFor;
use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub trait MessageSerializer {
    fn serialize(&self, message: &Message, buffer: &[u8]);
    fn deserialize(&self, buffer: &[u8]) -> Message;
}

pub type MessageSerializerImpl = dyn ImplFor<SubSystem=dyn MessageSerializer>;