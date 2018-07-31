//! The routes of [`server`].
//! 
//! Last Moddified --- 2018/07/30
//! 
//! # Authors
//! 
//! * daniel.bechaz@gmail.com

use utils::{card::{Id, Card,}, db,};
use rocket::State;
use rocket_contrib::Json;
use mongodb::{db::Database, error,};
use std::sync::RwLock;

#[post("/<title>/<description>")]
pub fn create_new(title: String, description: String, db: State<RwLock<Database>>) -> error::Result<Json<Id>> {
    let db = db.write().expect("Database lock poisoned");

    db::create_card(&*db, title, description).map(Json)
}

#[get("/<id>")]
pub fn get_card(id: Id, db: State<RwLock<Database>>) -> error::Result<Json<Card>> {
    let db = db.read().expect("Database lock poisoned");
    
    db::get_card(&*db, id).map(Json)
}

#[delete("/<id>")]
pub fn delete_card(id: Id, db: State<RwLock<Database>>) -> error::Result<()> {
    let db = db.write().expect("Database lock poisoned");
    
    db::delete_card(&*db, id)
}
