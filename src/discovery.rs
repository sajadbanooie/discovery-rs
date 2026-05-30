use crate::discovery::crypto::Crypto;
use crate::discovery::data_model::NodeProfile;
use crate::discovery::db::Database;
use crate::discovery::message::{Message, MessageType};
use crate::discovery::message_serializer::MessageSerializer;
use crate::discovery::session::{Session, SessionStorage};
use crate::discovery::transport::Transport;
use std::error::Error;

pub mod crypto;
pub mod data_model;
pub mod db;
pub mod message;
pub mod message_serializer;
pub mod message_types;
pub mod methods;
pub mod session;
pub mod settings;
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

type Result<T> = std::result::Result<T, Box<dyn Error>>;
pub trait ImplFor {
    type SubSystem;
    async fn init(settings: &settings::Section) -> Result<Box<Self::SubSystem>>;
    async fn fini(subsystem: Box<Self::SubSystem>);
}

pub struct Interface {
    pub message_serializer: Box<dyn MessageSerializer>,
    pub transport: Box<dyn Transport>,
}
pub struct SubSystems {
    pub crypto: Box<dyn Crypto>,
    pub db: Box<dyn Database>,
    pub session_storage: Box<dyn SessionStorage>,
}

pub struct MethodContext<'a> {
    node_profile: &'a NodeProfile,
    sub_systems: &'a mut SubSystems,
    session: &'a mut Session,
    interface: &'a mut Interface,
}

pub trait MethodHandler<'a> {
    async fn handle(&mut self, context: MethodContext<'a>, msg: &Message) -> Result<Message>;
}
