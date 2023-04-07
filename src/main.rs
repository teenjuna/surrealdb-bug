use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use std::fs::File;
use surrealdb::engine::any::connect;
use surrealdb::engine::any::Any;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Stop {
    pub id: Thing,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let file = File::open("stops.json")?;
    let data: Vec<Stop> = serde_json::from_reader(&file)?;

    let db = get_db().await?;

    for stop in data.iter() {
        let _: Option<Stop> = db.create("stop").content(stop).await?;
    }

    tracing::debug!("Now exporting...");
    db.export("/tmp/export.surql").await?;

    Ok(())
}

/// Creates a new in-memory database with some schema.
pub async fn get_db() -> Result<Surreal<Any>> {
    let db = connect("mem://").await?;
    db.use_ns("test").use_db("test").await?;
    db.query("DEFINE TABLE stop SCHEMAFULL").await?.check()?;
    Ok(db)
}
