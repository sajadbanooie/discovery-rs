mod http;
pub mod tcp;
mod tls;

use crate::discovery::ImplFor;
use crate::discovery::data_model::INetAddr;
use crate::discovery::session::Session;
use std::error::Error;
use std::fmt::{Display, Formatter};

type Result<T> = std::result::Result<T, Box<dyn Error>>;
pub trait TransportSession: std::any::Any {}
pub trait Transport {
    fn init_session<'a>(
        &'a self,
    ) -> Box<dyn Future<Output = Result<Box<dyn TransportSession>>> + 'a>;
    fn send<'a>(
        &'a self,
        session: &'a mut dyn TransportSession,
        buff: &'a [u8],
    ) -> Box<dyn Future<Output = Result<()>> + 'a>;
    fn recv<'a>(
        &'a self,
        session: &'a mut dyn TransportSession,
        buff: &'a mut [u8]
    ) -> Box<dyn Future<Output = Result<usize>> + 'a>;
    fn close_session<'a>(
        &'a self,
        session: Box<dyn TransportSession>,
    ) -> Box<dyn Future<Output = Result<()>> +'a>;
}

#[derive(Debug)]
struct InvalidSessionError {

}

impl Error for InvalidSessionError {}
impl Display for InvalidSessionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "TransportIOError")
    }
}

pub type TransportImpl = dyn ImplFor<SubSystem = dyn Transport>;
