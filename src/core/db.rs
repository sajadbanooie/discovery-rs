use crate::core::data_model::{IDHash, INetAddr, KeyType, MSGSignature, PrivateKey, PubKey};
use crate::core::settings::Settings;
use std::error::Error;
use crate::core::crypto::Crypto;
use crate::core::ImplFor;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub struct DBRecord {
    pub id_hash: IDHash,
    pub pub_key: PubKey,
    pub inet_addr: INetAddr,
    pub inet_addr_signature: MSGSignature,
    pub attributes: u32,
    pub trust_score: i32,
    pub reliability_score: i32,
}
pub trait Database {
    fn inc_ref(&self) -> Box<dyn Future<Output = Result<()>>>;
    fn dec_ref(&self) -> Box<dyn Future<Output = Result<()>>>;
    fn find(&self, id_hash: &IDHash) -> Box<dyn Future<Output = Result<()>>>;
    fn fetch(&self, id_hash: &IDHash) -> Box<dyn Future<Output = Result<Box<DBRecord>>> + Unpin>;
    fn store(&self, record: &DBRecord) -> Box<dyn Future<Output = Result<()>>>;
}
pub type DatabaseImpl = dyn ImplFor<SubSystem=dyn Database>;