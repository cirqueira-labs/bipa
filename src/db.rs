use chrono::prelude::DateTime;
use chrono::Utc;
use core::panic;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const ENDPOINT_URL: &str = "https://mempool.space/api/v1/lightning/nodes/rankings/connectivity";
const DATABASE_URL: &str = "sqlite://db.sqlite";

pub type Nodes = Vec<Node>;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub public_key: String,
    pub alias: String,
    pub capacity: f64,
    pub first_seen: u64,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MyNode {
    pub public_key: String,
    pub alias: String,
    pub capacity: f64,
    pub first_seen: String,
}

pub async fn get_data() -> Result<String, serde_json::Error> {
    let client = Client::new();
    let response: String = client
        .get(ENDPOINT_URL)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let nodes: Nodes = serde_json::from_str(&response).unwrap();

    let pool = db_init().await;

    insert_data(&pool, &nodes).await;

    let return_nodes = select_data(&pool).await;

    return_nodes
}

async fn select_data(pool: &Pool<Sqlite>) -> Result<String, serde_json::Error> {
    let nodes: Vec<MyNode> = sqlx::query_as("SELECT * FROM nodes")
        .fetch_all(pool)
        .await
        .map(axum::Json)
        .unwrap()
        .to_vec();

    let json_nodes = serde_json::to_string(&nodes)?;

    Ok(json_nodes)
}

async fn db_init() -> Pool<Sqlite> {
    if !Sqlite::database_exists(DATABASE_URL).await.unwrap_or(false) {
        match Sqlite::create_database(DATABASE_URL).await {
            Ok(_) => println!("Database created"),
            Err(e) => println!("Error creating database: {}", e),
        }
    } else {
        println!("Database already ok");
    }

    let pool = SqlitePool::connect(DATABASE_URL).await.unwrap();

    pool
}

async fn insert_data(pool: &Pool<Sqlite>, nodes: &Nodes) {
    let result = sqlx::query("DROP TABLE nodes").execute(pool).await;

    let result = sqlx::query(
        "CREATE TABLE IF NOT EXISTS nodes (
        public_key TEXT NOT NULL,
        alias TEXT NOT NULL,
        first_seen TEXT NOT NULL,
        capacity FLOAT NOT NULL
    )",
    )
    .execute(pool)
    .await;

    match result {
        Ok(_) => println!("Table created"),
        Err(error) => println!("Error creating table: {}", error),
    }

    for node in nodes {
        let d = UNIX_EPOCH + Duration::from_secs(node.first_seen);
        let datetime = DateTime::<Utc>::from(d);
        let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string();

        let result = sqlx::query(
            "INSERT INTO nodes (public_key, alias, capacity, first_seen)
        VALUES ($1, $2, $3, $4)",
        )
        .bind(&node.public_key)
        .bind(&node.alias)
        .bind(&node.capacity / 100_000_000f64)
        .bind(timestamp_str)
        .execute(pool)
        .await;

        match result {
            Ok(_) => println!("Row inserted"),
            Err(error) => panic!("{error}"),
        }
    }

    println!("Nodes inserted");
}
