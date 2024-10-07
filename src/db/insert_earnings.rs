use mongodb::Collection;
use crate::models::earnings_history::EarningsHistory;
use mongodb::bson::oid::ObjectId;

pub async fn _insert_earnings(
    collection: &Collection<EarningsHistory>,
    earnings_data: Vec<EarningsHistory>,
) -> mongodb::error::Result<Vec<ObjectId>> {
    let mut inserted_ids = Vec::new();
    
    for earnings in earnings_data {
        let result = collection.insert_one(earnings).await?;
        let id = result.inserted_id.as_object_id().unwrap();
        inserted_ids.push(id);
    }

    Ok(inserted_ids)
}
