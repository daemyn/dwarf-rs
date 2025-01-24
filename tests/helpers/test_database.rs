use sqlx::{Connection, PgConnection, PgPool, Executor};
use uuid::Uuid;

pub struct TestDatabase {
    pub pool: PgPool,
    base_db_url: String,
    test_db_name: String,
}

impl TestDatabase {
    pub async fn new() -> Self {
        let base_db_url = "postgres://postgres:postgres@localhost:5432".to_string();
        let test_db_name = format!("dwarf_test_db_{}", Uuid::new_v4());

        let mut connection = PgConnection::connect(&base_db_url)
            .await
            .expect("Failed to connect to PostgreSQL");

        connection
            .execute(format!(r#"CREATE DATABASE "{}""#, test_db_name).as_str())
            .await
            .expect("Failed to create test database");

        let test_db_url = format!("{}/{}", base_db_url, test_db_name);

        let pool = PgPool::connect(&test_db_url)
            .await
            .expect("Failed to connect to test database");

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations");

        Self {
            pool,
            base_db_url,
            test_db_name,
        }
    }
}

impl Drop for TestDatabase {
    fn drop(&mut self) {
        let base_db_url = self.base_db_url.clone();
        let test_db_name = self.test_db_name.clone();

        // This is a blocking operation: Spawn a separate thread to drop the database
        std::thread::spawn(move || {
            // Drop is sync: use tokio runtime to be able to await for async operations
            let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
            rt.block_on(async {
                let mut connection = PgConnection::connect(&base_db_url)
                    .await
                    .expect("Failed to reconnect to PostgreSQL");

                connection
                    .execute(format!(r#"DROP DATABASE "{}" WITH (FORCE)"#, test_db_name).as_str())
                    .await
                    .expect("Failed to drop test database");
            });
        })
        .join()
        .expect("Failed to drop test database");
    }
}