//! Provides logic for database operations.
//! 
//! Last Moddified --- 2018/07/30
//! 
//! # Authors
//! 
//! * daniel.bechaz@gmail.com

use card::*;
use mongodb::{db::{Database, ThreadedDatabase,}, error,};
use bson::{Bson, from_bson,};

pub fn create_card(db: &Database, title: String, description: String) -> error::Result<Id> {
    let event = db.collection("event");

    event.insert_one(doc! {
        "title" : title,
        "description" : description,
    }, None).map(|res| res.inserted_id.unwrap().into())
}

pub fn get_card(db: &Database, id: Id) -> error::Result<Card> {
    let event = db.collection("event");

    event.find_one(Some(doc! { "_id" : Into::<Bson>::into(id), }), None)
    .and_then(|res| res.map(|doc| from_bson(Bson::from(doc)).expect("Failed to deserialise Card")).ok_or(
        error::Error::DefaultError("The specified document could not be found".to_owned())
    ))
}

pub fn delete_card(db: &Database, id: Id) -> error::Result<()> {
    let event = db.collection("event");

    event.delete_one(doc! { "_id" : Into::<Bson>::into(id), }, None).map(|_| ())
}
