use crate::core::data_model::NodeProfile;
use crate::core::message::{Message, MessageType};
use crate::core::session::Session;
use crate::core::methods::{MethodContext, MethodHandler};
use crate::core::{SubSystems, message_types};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
struct InvalidMethodError;

impl Error for InvalidMethodError {

}

impl Display for InvalidMethodError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid method!")
    }
}

pub struct InvalidMethodHandler;

impl<'a> MethodHandler<'a> for InvalidMethodHandler {
    async fn handle(
        &mut self,
        context: MethodContext<'a>,
        msg: &Message,
    ) -> Result<Message, Box<dyn Error>> {
        Err(Box::new(InvalidMethodError {}))
    }
}
