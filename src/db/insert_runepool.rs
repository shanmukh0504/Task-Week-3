use mongodb::Collection;
use crate::models::runepool_history::RunePoolHistory;
use mongodb::error::Result;

pub async fn _insert_runepool_history(collection: &Collection<RunePoolHistory>, data: Vec<RunePoolHistory>) -> Result<()> {
    collection.insert_many(data).await?;
    Ok(())
}
