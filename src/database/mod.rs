use surrealdb::{engine::local:: Mem, Surreal};

pub async fn start_db() -> surrealdb::Result<()> {
    let db = Surreal::new::<Mem>(()).await?;
    db.use_ns("dembed").use_db("dembed").await?;

    Ok(())
}

#[derive(Debug)]
pub enum UserType {
    Free(String),
    Premium(String),
}

#[allow(unused)]
pub async fn get_user_type(user_id: u64) -> anyhow::Result<UserType> {
    Ok(UserType::Premium("todo!".to_string()))
}
