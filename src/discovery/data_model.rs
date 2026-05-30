pub type ID = String;
pub type IDHash = String;

#[derive(Debug)]
pub enum KeyType {
    RSA,
    DSA,
    ED25519,
}

#[derive(Debug)]
pub struct PubKey {
    key_type: KeyType,
    key: String,
}

#[derive(Debug)]
pub struct PrivateKey {
    key_type: KeyType,
    key: String,
}

#[derive(Debug)]
pub enum INetType {
    IPV4,
    IPV6,
}

#[derive(Debug)]
pub struct INetAddr {
    inet_type: INetType,
    addr: String,
}

#[derive(Debug)]
pub struct MSGSignature {
    key_type: KeyType,
    sig: String,
}

#[derive(Debug)]
pub struct NodeProfile {
    node_name: String,
    node_id: ID,
    id_hash: IDHash,
    pubkey: PubKey,
    private_key: PrivateKey,
}