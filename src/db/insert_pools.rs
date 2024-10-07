use mongodb::Collection;
use crate::models::pools_history::PoolHistory;
use mongodb::bson::oid::ObjectId;
use std::io;

pub async fn _insert_pools(
    collection: &Collection<PoolHistory>,
    pools_data: Vec<PoolHistory>,
    earnings_ids: &Vec<ObjectId>,
) -> mongodb::error::Result<()> {
    if earnings_ids.is_empty() {
        return Err(mongodb::error::Error::from(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Earnings ID not found",
        )));
    }

    let earnings_id = &earnings_ids[0];
    for mut pool in pools_data {
        pool.earnings_id = earnings_id.clone();
        pool.pool = pool.pool.trim_matches('"').to_string();
        collection.insert_one(pool).await?;
    }

    Ok(())
}
