use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};
use std::env;
#[derive(Clone)]
pub struct Db {
    pub pool: Pool<MySql>,
}

pub enum Env {
    Production,
    Test,
}

impl Db {
    pub async fn new(env: Env) -> Db {
        match env {
            Env::Production => {
                let pool = MySqlPoolOptions::new()
                    .max_connections(10)
                    .connect(&env::var("DATABASE_URL").unwrap_or_else(|_| {
                        "mysql://root:password@localhost/restaurant".to_string()
                    }))
                    .await
                    .unwrap();
                Db { pool }
            }
            Env::Test => {
                let pool = MySqlPoolOptions::new()
                    .max_connections(10)
                    .connect(&env::var("TEST_DATABASE_URL").unwrap_or_else(|_| {
                        "mysql://root:password@localhost/restaurant_test".to_string()
                    }))
                    .await
                    .unwrap();
                Db { pool }
            }
        }
    }
}
