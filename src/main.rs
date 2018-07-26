//! The backend webserver of MeetMon.
//! 
//! Last Moddified --- 2018/07/26
//! 
//! # Authors
//! 
//! * daniel.bechaz@gmail.com

#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

fn main() {
    rocket::ignite()
    .launch();
}
