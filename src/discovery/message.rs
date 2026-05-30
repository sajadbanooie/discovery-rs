use std::collections::hash_map::HashMap;
use std::fmt::{write, Display, Formatter};
use super::data_model::*;

#[derive(Debug, Copy, Clone)]
pub struct MessageType {
    pub code: u8,
    pub repr: &'static str,
}

impl Display for MessageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}({})",self.repr, self.code)
    }
}

#[derive(Debug)]
pub enum MsgField {
    FieldInt(u32),
    FieldStr(String),
    FieldPubkey(PubKey),
    FieldInetAddr(INetAddr),
    FieldSig(MSGSignature),
    FieldId(ID),
    FieldIdHash(IDHash),
}

impl Display for MsgField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MsgField::FieldInt(_) => {write!(f, "int")}
            MsgField::FieldStr(_) => {write!(f, "str")}
            MsgField::FieldPubkey(_) => {write!(f, "pubkey")}
            MsgField::FieldInetAddr(_) => {write!(f, "inet_addr")}
            MsgField::FieldSig(_) => {write!(f, "sig")}
            MsgField::FieldId(_) => {write!(f, "id")}
            MsgField::FieldIdHash(_) => {write!(f, "id_hash")}
        }
    }
}


#[derive(Debug)]
pub struct Message {
    pub message_type: MessageType,
    fields: HashMap<String, MsgField>,
}

impl Message {
    pub fn empty_message(message_type: MessageType) -> Self {
        Message {
            message_type,
            fields: HashMap::new(),
        }
    }
    pub fn add_field(& mut self, field_name: String, value: MsgField) {
        self.fields.insert(field_name, value);
    }
    pub fn get_field(&self, field_name: &str) -> Option<&MsgField> {
        self.fields.get(field_name)
    }
}
