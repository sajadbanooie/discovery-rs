use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::core::message::MsgField;

mod invalid_method;
mod query_method;
mod auth_method;


pub use invalid_method::InvalidMethodHandler;
pub use query_method::QueryMethodHandler;
pub use auth_method::AuthMethodHandler;

#[derive(Debug)]
struct FieldNotPresentError<'a>{
    field_name: &'a str    
}

impl Error for FieldNotPresentError<'_> {

}

impl Display for FieldNotPresentError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "field {} not present in message.", self.field_name)
    }
}


#[derive(Debug)]
struct InvalidFieldType<'a>{
    field_name: &'a str,
    expected: MsgField,
    found: MsgField
}

impl Error for InvalidFieldType<'_> {

}

impl Display for InvalidFieldType<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "field {} has type {} instead of {}", 
               self.field_name, self.found, self.expected)
    }
}