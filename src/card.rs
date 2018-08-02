//! Defines a [`Card`] and its necessary components.
//! 
//! Last Moddified --- 2018/07/30
//! 
//! # Authors
//! 
//! * daniel.bechaz@gmail.com

use bson::Bson;
use rocket::{request::FromParam, http::RawStr,};
use serde_json::{self, from_str,};

#[derive(Serialize, Deserialize,)]
pub struct Id(Bson);

impl From<Bson> for Id {
    #[inline]
    fn from(from: Bson) -> Self { Id(from) }
}

impl Into<Bson> for Id {
    #[inline]
    fn into(self) -> Bson { self.0 }
}

impl<'a> FromParam<'a> for Id {
    type Error = serde_json::Error;

    fn from_param(param: &'a RawStr) -> Result<Self, Self::Error> {
        from_str(param.as_str())
    }
}

#[derive(Serialize, Deserialize,)]
pub struct Card {
    pub id: Id,
    pub title: String,
    pub description: String,
}
