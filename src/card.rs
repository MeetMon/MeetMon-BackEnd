//! Defines a [`Card`] and its necessary components.
//! 
//! Last Moddified --- 2018/07/30
//! 
//! # Authors
//! 
//! * daniel.bechaz@gmail.com

use bson::{self, Bson,};
use rocket::{request::FromParam, http::RawStr,};

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
    type Error = bson::EncoderError;

    fn from_param(param: &'a RawStr) -> Result<Self, Self::Error> {
        bson::to_bson(param.as_str()).map(Id)
    }
}

#[derive(Serialize, Deserialize,)]
pub struct Card {
    pub id: Id,
    pub title: String,
    pub description: String,
}
