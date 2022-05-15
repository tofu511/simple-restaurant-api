pub mod item_repository;

use crate::repository::item_repository::ItemRepository;

pub trait Repositories {
    type ItemRepo: ItemRepository;
    fn item_repository(&self) -> &Self::ItemRepo;
}
