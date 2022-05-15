use crate::{controllers::item, modules::ModulesImpl};
use axum::{
    extract::Extension,
    routing::{delete, get, post},
    Router,
};

pub async fn router(modules: ModulesImpl) -> Router {
    let item_router = Router::new()
        .route("/:table_number/item", post(item::add_item))
        .route("/:table_number/items", get(item::query_items))
        .route("/:table_number/items/:item_id", get(item::query_item))
        .route("/:table_number/items/:item_id", delete(item::delete_item))
        .layer(Extension(modules));

    Router::new().nest("/tables", item_router)
}
