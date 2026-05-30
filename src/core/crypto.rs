use std::error::Error;
use crate::core::data_model::{KeyType, MSGSignature, PrivateKey, PubKey};
use crate::core::ImplFor;

type Result<T> = std::result::Result<T, Box<dyn Error>>;
pub trait Crypto {
    fn create_key_pair(&self, key_type: KeyType) -> Result<(PubKey, PrivateKey)>;
    fn verify_signature(&self, pub_key: &PubKey, msg_signature: &MSGSignature) -> Result<bool>;
    fn sign(&self, private_key: &PrivateKey, msg: &str) -> Result<MSGSignature>;
    fn pub_key_decode(&self, pub_key: &PubKey, cipher: &str) -> Result<String>;
    fn private_key_encode(&self, private_key: &PrivateKey, msg: &str) -> Result<String>;
}

pub type CryptoImpl = dyn ImplFor<SubSystem = dyn Crypto>;