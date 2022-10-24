use demo_db::{Database, database::Pooled};


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

    pub fn db_conn(&self) -> Pooled {
        self.db.conn()
    }
    
}