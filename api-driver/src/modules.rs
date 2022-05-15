use api_adapter::repository_impl::RepositoriesImpl;
use api_core::repository::Repositories;
use api_usecase::usecase::item::ItemUseCase;
pub trait Modules {
    type Repositories: Repositories;

    fn item_usecase(&self) -> &ItemUseCase<Self::Repositories>;
}

#[derive(Clone)]
pub struct ModulesImpl {
    item_usecase: ItemUseCase<RepositoriesImpl>,
}

impl Modules for ModulesImpl {
    type Repositories = RepositoriesImpl;

    fn item_usecase(&self) -> &ItemUseCase<Self::Repositories> {
        &self.item_usecase
    }
}

impl ModulesImpl {
    pub async fn new() -> ModulesImpl {
        let repositories_module = RepositoriesImpl::new().await;
        let item_usecase = ItemUseCase::new(repositories_module);
        ModulesImpl { item_usecase }
    }

    pub async fn test() -> ModulesImpl {
        let repositories_module = RepositoriesImpl::test().await;
        let item_usecase = ItemUseCase::new(repositories_module);
        ModulesImpl { item_usecase }
    }
}
