use mongodb::Collection;
use crate::models::depth_history::DepthHistory;
use mongodb::error::Result;

pub async fn _insert_depth_history(collection: &Collection<DepthHistory>, data: Vec<DepthHistory>) -> Result<()> {
    collection.insert_many(data).await?;
    Ok(())
}