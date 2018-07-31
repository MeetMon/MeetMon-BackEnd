//! The logic of the backend for [`server`].
//! 
//! Last Moddified --- 2018/07/30
//! 
//! # Authors
//! 
//! * daniel.bechaz@gmail.com

#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate bson;
extern crate mongodb;
extern crate rocket;
extern crate serde_json;

pub mod card;
pub mod db;
