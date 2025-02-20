use std::env;

use mysql::*;

use once_cell::sync::OnceCell;

static DB_POOL: OnceCell<Pool> = OnceCell::new();

async fn initialize() {
    if DB_POOL.get().is_some() {
        // we already have created the pool
        return;
    }
    let my_host: String = env::var("MYSQL_HOST").unwrap_or("localhost".to_string());
    let my_user: String = env::var("MYSQL_USER").unwrap_or("root".to_string());
    let my_pass: String = env::var("MYSQL_PASS").unwrap_or("root".to_string());
    let my_port: String = env::var("MYSQL_PORT").unwrap_or("3306".to_string());
    let my_conn: String = env::var("MYSQL_CONNECTION").unwrap_or("mysql".to_string());
    let my_db: String = env::var("MYSQL_DATABASE").unwrap_or("stats".to_string());
    let connection_str: String =
        format!("{my_conn}://{my_user}:{my_pass}@{my_host}:{my_port}/{my_db}");
    let pool = Pool::new(connection_str.as_str()).unwrap();
    rocket::debug!("Successfully connected to MySQL host!");
    DB_POOL.get_or_init(|| pool);
}

pub async fn get_conn() -> &'static OnceCell<Pool> {
    initialize().await;
    &DB_POOL
}
