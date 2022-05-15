use api_core::{
    domain::item::Item,
    error::Error,
    repository::{item_repository::ItemRepository, Repositories},
};

#[derive(Clone)]
pub struct ItemUseCase<R: Repositories> {
    repositories: R,
}

impl<R: Repositories> ItemUseCase<R> {
    pub fn new(repositories: R) -> Self {
        Self { repositories }
    }
}

impl<R: Repositories> ItemUseCase<R> {
    pub async fn query_item(&self, table_number: u32, item_id: u32) -> Result<Item, Error> {
        self.repositories
            .item_repository()
            .find_one(table_number, item_id)
            .await
    }

    pub async fn query_items(&self, table_number: u32) -> Result<Vec<Item>, Error> {
        self.repositories
            .item_repository()
            .find_all(table_number)
            .await
    }

    pub async fn add_item(&self, table_number: u32, item: Item) -> Result<u64, Error> {
        self.repositories
            .item_repository()
            .create(table_number, item)
            .await
    }

    pub async fn delete_item(&self, table_number: u32, item_id: u32) -> Result<(), Error> {
        self.repositories
            .item_repository()
            .delete(table_number, item_id)
            .await
    }
}
