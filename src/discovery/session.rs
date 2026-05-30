use std::error::Error;
use crate::discovery::data_model::{IDHash, INetAddr};
use crate::discovery::db::{DBRecord, Database};
use crate::discovery::ImplFor;

type TransportSession = [u8];

pub struct Session {
    pub id_hash: IDHash,
    pub inet_addr: INetAddr,
    pub is_authenticated: bool,
    pub transport_session: Box<TransportSession>
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub trait SessionStorage {
    fn fetch(&self, id_hash: &IDHash) -> Box<dyn Future<Output = Result<Box<Session>>>>;
    fn store(&self, session: Box<Session>) -> Box<dyn Future<Output = Result<()>>>;
}

pub type SessionStorageImpl = dyn ImplFor<SubSystem=dyn SessionStorage>;