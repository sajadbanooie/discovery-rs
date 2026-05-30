use crate::discovery::data_model::NodeProfile;
use crate::discovery::db::DBRecord;
use crate::discovery::message::{Message, MessageType, MsgField};
use crate::discovery::message_types;
use crate::discovery::methods::{FieldNotPresentError, InvalidFieldType};
use crate::discovery::session::Session;
use crate::discovery::{MethodContext, MethodHandler, SubSystems};
use std::error::Error;

struct QueryMethodHandler;
impl<'a> MethodHandler<'a> for QueryMethodHandler {
    async fn handle(
        &mut self,
        context: MethodContext<'a>,
        msg: &Message,
    ) -> Result<Message, Box<dyn Error>> {
        let requested_id_hash = msg.get_field("id_hash");
        let requested_id_hash = match requested_id_hash {
            Some(MsgField::FieldIdHash(r)) => Some(r),
            Some(_) => None,
            None => None,
        };
        if requested_id_hash.is_none() {
            return Err(Box::new(FieldNotPresentError {
                field_name: "id_hash",
            }));
        }
        
        let sub_systems = context.sub_systems;

        let future_work = sub_systems.db.inc_ref();
        let future_work = Box::into_pin(future_work);
        future_work.await?;

        let requested_id_hash = requested_id_hash.unwrap();
        let future_work = sub_systems.db.fetch(requested_id_hash);
        // let future_work = Box::into_pin(future_work);
        let record: Box<DBRecord> = future_work.await?;

        let mut r = Message::empty_message(message_types::ANS);
        r.add_field(
            "addr".to_string(),
            MsgField::FieldInetAddr((*record).inet_addr),
        );
        Ok(r)
    }
}
