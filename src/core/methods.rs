


pub struct MethodContext<'a> {
    pub node_profile: &'a crate::core::data_model::NodeProfile,
    pub sub_systems: &'a mut crate::core::SubSystems,
    pub session: &'a mut crate::core::session::Session,
    pub interface: &'a mut crate::core::Interface,
}

pub trait MethodHandler<'a> {
    async fn handle(&mut self, context: MethodContext<'a>, msg: &crate::core::message::Message) -> crate::core::Result<crate::core::message::Message>;
}
