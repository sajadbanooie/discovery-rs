use crate::discovery::data_model::NodeProfile;
use crate::discovery::message::{Message, MessageType};
use crate::discovery::methods::InvalidMethodError;
use crate::discovery::session::Session;
use crate::discovery::{MethodContext, MethodHandler, SubSystems, message_types};
use std::error::Error;
use std::fmt::{Display, Formatter};

struct InvalidMethodHandler;

impl<'a> MethodHandler<'a> for InvalidMethodHandler {
    async fn handle(
        &mut self,
        context: MethodContext<'a>,
        msg: &Message,
    ) -> Result<Message, Box<dyn Error>> {
        Err(Box::new(InvalidMethodError {}))
    }
}
