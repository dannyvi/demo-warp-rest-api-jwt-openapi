use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::{Pool, PooledConnection};

type PgPool = Pool<ConnectionManager<PgConnection>>;

pub type Connection = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    /// Creates a repo using default configuration for the underlying connection pool.
    pub fn new(database_url: String) -> Self {
        Self::from_pool_builder(database_url.as_str(), r2d2::Builder::default())
    }

    /// Creates a repo with a pool builder, allowing you to customize
    /// any connection pool configuration.
    pub fn from_pool_builder(
        database_url: &str,
        builder: r2d2::Builder<ConnectionManager<PgConnection>>,
    ) -> Self {
        let manager = ConnectionManager::new(database_url);
        let pool = builder
            .build(manager)
            .expect("could not initiate test db pool");
        Database { pool }
    }

    pub fn conn(&self) -> Connection {
        self.pool.get().unwrap()
    }
}