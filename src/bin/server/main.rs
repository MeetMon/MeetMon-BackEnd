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

static MONGO_HOST: &str = "ds249311.mlab.com";
static MONGO_PORT: u16 = 49311;
static USERNAME: &str = "meetmon-test";
static PASSWORD: &str = "1testaccount";
static DB: &str = "meetmon";


fn main() {
    let client = Client::connect(MONGO_HOST,MONGO_PORT)
        .expect("Failed to connect to database");
    let database = client.db(DB);
        
    database.auth(USERNAME, PASSWORD)
        .expect("Authentication failed");

    let database = RwLock::new(database);

    rocket::ignite()
    .mount("/event", routes![
        create_new, get_card, delete_card,
    ])
    .manage(database)
    .launch();
}
