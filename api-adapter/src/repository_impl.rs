use crate::{
    persistence::mysql::{Db, Env},
    repository::item_repository_impl::ItemRepositoryImpl,
};
use api_core::repository::Repositories;

#[derive(Clone)]
pub struct RepositoriesImpl {
    item_repository: ItemRepositoryImpl,
}

impl Repositories for RepositoriesImpl {
    type ItemRepo = ItemRepositoryImpl;

    fn item_repository(&self) -> &Self::ItemRepo {
        &self.item_repository
    }
}

impl RepositoriesImpl {
    pub async fn new() -> Self {
        let db = Db::new(Env::Production).await;
        let item_repository = ItemRepositoryImpl::new(db);
        Self { item_repository }
    }

    pub async fn test() -> Self {
        let db = Db::new(Env::Test).await;
        let item_repository = ItemRepositoryImpl::new(db);
        Self { item_repository }
    }
}
