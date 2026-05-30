use crypto::Crypto;
use data_model::NodeProfile;
use db::Database;
use message::Message;
use message_serializer::MessageSerializer;
use session::{Session, SessionStorage};
use transport::Transport;

use std::error::Error;

pub mod crypto;
pub mod data_model;
pub mod db;
pub mod message;
pub mod message_serializer;
pub mod message_types;
pub mod session;
pub mod settings;
pub mod methods;
pub mod transport;

pub enum ProtocolMethods {
    Invalid = -1,
    Query = 1,
    Auth,
    Join,
    Update,
    New,
    Beacon,
}

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
pub trait ImplFor {
    type SubSystem;
    async fn init(settings: &settings::Section) -> Result<Box<Self::SubSystem>>;
    async fn fini(subsystem: Box<Self::SubSystem>);
}

pub struct Interface {
    pub message_serializemessage_serializerr: Box<dyn MessageSerializer>,
    pub transport: Box<dyn Transport>,
}
pub struct SubSystems {
    pub crypto: Box<dyn Crypto>,
    pub db: Box<dyn Database>,
    pub session_storage: Box<dyn SessionStorage>,
}
