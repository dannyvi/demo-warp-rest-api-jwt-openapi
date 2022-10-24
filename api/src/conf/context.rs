use std::convert::Infallible;

use demo_db::{Database, database::Connection};
use warp::Filter;


#[derive(Clone)]
pub struct Context {
    db: Database,
    jwt: String,
}

impl Context {
    pub fn new(database_url: String, secret: String) -> Self {
        let db = Database::new(database_url);
        Self { db, jwt: secret }
    }

    pub fn db_conn(&self) -> Connection {
        self.db.conn()
    }
   
    pub fn filter(self) -> impl Filter<Extract = (Context,), Error = Infallible> + Clone {
        warp::any().map(move || self.clone())
    } 
}