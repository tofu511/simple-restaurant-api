use crate::domain::item::Item;
use crate::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait ItemRepository {
    async fn find_all(&self, table_number: u32) -> Result<Vec<Item>, Error>;
    async fn find_one(&self, table_number: u32, item_id: u32) -> Result<Item, Error>;
    async fn create(&self, table_number: u32, item: Item) -> Result<u64, Error>;
    async fn delete(&self, table_number: u32, item_id: u32) -> Result<(), Error>;
}
