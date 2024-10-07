use mongodb::Collection;
use crate::models::swaps_history::SwapHistory;
use mongodb::error::Result;

pub async fn _insert_swap_history(collection: &Collection<SwapHistory>, data: Vec<SwapHistory>) -> Result<()> {
    collection.insert_many(data).await?;
    Ok(())
}
