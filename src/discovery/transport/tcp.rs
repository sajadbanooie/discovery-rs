use crate::discovery::transport::{InvalidSessionError, Transport, TransportSession};
use crate::discovery::Result;
use crate::discovery::{settings, ImplFor};

use std::error::Error;

use futures::executor::block_on;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub struct TcpTransportPlugin;
pub struct TcpTransport {
    pub(super) listener: TcpListener,
}

pub struct TcpSession {
    pub(super) addr: std::net::SocketAddr,
    pub(super) stream: TcpStream,
}

impl TransportSession for TcpSession {

}

impl ImplFor for TcpTransportPlugin {
    type SubSystem = TcpTransport;
    async fn init(settings: &settings::Section) -> Result<Box<TcpTransport>> {
        Ok(Box::new(TcpTransport {
            listener: TcpListener::bind("localhost:8080").await?,
        }))
    }

    async fn fini(subsystem: Box<TcpTransport>) {}
}

impl Transport for TcpTransport {
    fn init_session<'a>(
        &'a self,
    ) -> Box<dyn Future<Output = Result<Box<dyn TransportSession>>> + 'a>
    {
        Box::new(async {
            let (stream, addr) = self.listener.accept().await?;
            
            let session: Box<dyn TransportSession> = Box::new(TcpSession { stream, addr });
            Ok(session)
        })
    }

    fn send<'a>(
        &'a self,
        session: &'a mut dyn TransportSession,
        buff: &'a [u8],
    ) -> Box<dyn Future<Output = Result<()>> + 'a> {
        Box::new(async {
            let session: &mut dyn std::any::Any = session;
            let session = session.downcast_mut::<TcpSession>();
            match session {
                None => { let e:Box<dyn Error> = Box::new(InvalidSessionError {});Err(e)}
                Some(session) => { session.stream.write(buff).await?; Ok(())}
            }
        })
    }

    fn recv<'a>(
        &'a self,
        session: &'a mut dyn TransportSession,
        buff: &'a mut [u8]
    ) -> Box<dyn Future<Output = Result<usize>> + 'a> {
        Box::new(async {
            let session: &mut dyn std::any::Any = session;
            let session = session.downcast_mut::<TcpSession>();
            match session {
                None => { let e:Box<dyn Error> = Box::new(InvalidSessionError {});Err(e)}
                Some(session) => { Ok(session.stream.read(buff).await?) }
            }
        })
    }

    fn close_session(
        &self,
        session: Box<dyn TransportSession>,
    ) -> Box<dyn Future<Output = Result<()>>> {
        Box::new(async move {
            let session: & dyn std::any::Any = session.as_ref();
            let session = session.downcast_ref::<TcpSession>();
            match session {
                None => {
                    let e: Box<dyn Error> = Box::new(InvalidSessionError {});
                    Err(e)
                }
                Some(_) => { Ok(()) }
            }
        })
    }
}