use surrealdb::{engine::local:: Mem, Surreal};

pub async fn start_db() -> surrealdb::Result<()> {
    let db = Surreal::new::<Mem>(()).await?;
    db.use_ns("dembed").use_db("dembed").await?;

    Ok(())
}
