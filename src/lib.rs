#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

pub const SERVER: &str = "0.0.0.0:8010";

mod routes;
