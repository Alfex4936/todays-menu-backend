#![feature(proc_macro_hygiene, decl_macro)]
extern crate serde_derive;
extern crate serde_json;

pub const SERVER: &str = "0.0.0.0:8010";

mod routes;