use serde::{Serialize, Deserialize};
use core::fmt;
use std::error::Error;

pub trait DatabaseConnection: Send + Sync{
    fn get_username(&self) -> &str;
    fn get_type(&self) -> ConnectionType;
}


#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum ConnectionType{
    Mongo, Postgres, Redis, None
}

#[derive(Debug)]
pub struct TryFromError(String);

impl Error for TryFromError {}
impl fmt::Display for TryFromError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"Attempted to use TryFrom and failed. Msg: {} ", self.0)
    }
}



impl TryFrom<usize> for ConnectionType {
    type Error = TryFromError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value{
            value if value == Self::Mongo as usize=> Ok(Self::Mongo),
            value if value == Self::Postgres as usize=> Ok(Self::Postgres),
            value if value == Self::Redis as usize=> Ok(Self::Redis),
            value if value == Self::None as usize=> Ok(Self::None),
            _ => Err(TryFromError("Could not cast usize to ConnectionType".to_owned()))
        }
    }
}