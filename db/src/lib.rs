#[macro_use]
extern crate diesel;

pub mod schema;
pub mod database;

pub use database::Database;

pub mod models;