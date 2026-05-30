use crate::discovery::data_model::NodeProfile;
use crate::discovery::db::DBRecord;
use crate::discovery::message::{Message, MsgField};
use crate::discovery::methods::FieldNotPresentError;
use crate::discovery::session::Session;
use crate::discovery::{MethodContext, MethodHandler, SubSystems, message_types};
use std::error::Error;

struct AuthMethodHandler;

impl<'a> MethodHandler<'a> for AuthMethodHandler {
    async fn handle(
        &mut self,
        context: MethodContext<'a>,
        msg: &Message,
    ) -> Result<Message, Box<dyn Error>> {
        let session = context.session;

        if session.is_authenticated {
            let r = Message::empty_message(message_types::AUTH_OK);
            return Ok(r);
        }
        let id_hash = msg.get_field("id_hash");
        let id_hash = match id_hash {
            Some(MsgField::FieldIdHash(r)) => Some(r),
            Some(_) => None,
            None => None,
        };

        let id_hash_sig = msg.get_field("id_hash_sig");
        let id_hash_sig = match id_hash_sig {
            Some(MsgField::FieldSig(r)) => Some(r),
            Some(other) => None,
            None => None,
        };

        if id_hash.is_none() || id_hash_sig.is_none() {
            return Err(Box::new(FieldNotPresentError {
                field_name: "id_hash",
            }));
        }

        if id_hash_sig.is_none() {
            return Err(Box::new(FieldNotPresentError {
                field_name: "id_hash_sig",
            }));
        }

        let id_hash = id_hash.unwrap();
        let id_hash_sig = id_hash_sig.unwrap();
        
        let sub_systems  = context.sub_systems;

        let future_work = sub_systems.db.inc_ref();
        let future_work = Box::into_pin(future_work);
        future_work.await?;

        let future_work = sub_systems.db.fetch(id_hash);
        let future_work = Box::into_pin(future_work);
        let record: Box<DBRecord> = future_work.await?;
        let pub_key = (*record).pub_key;

        if sub_systems.crypto.verify_signature(&pub_key, id_hash_sig)? {
            let r = Message::empty_message(message_types::AUTH_OK);
            Ok(r)
        } else {
            session.is_authenticated = true;
            let mut r = Message::empty_message(message_types::AUTH_FAILED);
            r.add_field(
                "reason".to_string(),
                MsgField::FieldStr("sig verification failed!".to_string()),
            );
            Ok(r)
        }
    }
}
