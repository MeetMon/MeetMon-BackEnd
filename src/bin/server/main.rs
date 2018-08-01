//! The backend webserver of MeetMon.
//! 
//! Last Moddified --- 2018/07/30
//! 
//! # Authors
//! 
//! * daniel.bechaz@gmail.com

#![feature(plugin)]
#![feature(decl_macro)]

#![plugin(rocket_codegen)]

use std::sync::RwLock;

extern crate utils;
extern crate rocket;
extern crate rocket_contrib;
extern crate mongodb;

mod routes;

use self::routes::*;
use mongodb::{Client, ThreadedClient, db::ThreadedDatabase,};

//static DATABASE_URI: &str = "mongodb://ds249311.mlab.com:49311/meetmon";
static DATABASE_URI: &str = "mongodb://meetmon-test:1testaccount@ds249311.mlab.com:49311/meetmon";
//static USERNAME: &str = "meetmon-test";
//static PASSWORD: &str = "1testaccount";
static COLLECTION: &str = "event";

fn main() {
    let client = Client::with_uri(DATABASE_URI)
        .expect("Failed to connect to database");
    let database = client.db(COLLECTION);
        
    /* database.auth(USERNAME, PASSWORD)
        .expect("Authentication failed"); */

    let database = RwLock::new(database);

    rocket::ignite()
    .mount("/event", routes![
        create_new, get_card, delete_card,
    ])
    .manage(database)
    .launch();
}
