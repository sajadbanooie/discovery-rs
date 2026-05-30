use discovery::discovery::ImplFor;
use discovery::discovery::transport::tcp::*;
use discovery::discovery::transport::*;

use std::collections::HashMap;
use std::error::Error;
use std::future::Future;
use std::result::Result;

#[tokio::main]
async fn main() {
    let settings: HashMap<String, String> = HashMap::new();
    let transport = TcpTransportPlugin::init(&settings).await;

    let transport: Box<dyn Transport> = transport.unwrap();

    let f: Box<dyn Future<Output = Result<Box<dyn TransportSession>, Box<dyn Error>>>> =
        transport.init_session();

    let f = Box::into_pin(f);

    let res: Result<Box<dyn TransportSession>, Box<dyn Error>> = f.await;
    match res {
        Ok(session) => {
            let f = transport.close_session(session);
            let f = Box::into_pin(f);
            f.await.unwrap();
        }
        Err(_err) => {}
    }


}
