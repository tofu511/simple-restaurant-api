use std::collections::HashMap;

use api_core::domain::item::Item;
use api_core::error::Error;
use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use chrono::Local;
use serde::{Deserialize, Serialize};

use crate::modules::{Modules, ModulesImpl};

#[axum_macros::debug_handler]
pub async fn query_item(
    Path(params): Path<HashMap<String, u32>>,
    Extension(modules): Extension<ModulesImpl>,
) -> Result<impl IntoResponse, StatusCode> {
    let result = modules
        .item_usecase()
        .query_item(
            params.get("table_number").unwrap().clone(),
            params.get("item_id").unwrap().clone(),
        )
        .await;
    match result {
        Ok(item) => {
            let json = JsonItemResponse::new(&item);
            Ok(Json(json))
        }
        Err(e) => match e {
            Error::ItemNotFoundError => Err(StatusCode::NOT_FOUND),
            _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    }
}

pub async fn query_items(
    Path(table_number): Path<u32>,
    Extension(modules): Extension<ModulesImpl>,
) -> Result<impl IntoResponse, StatusCode> {
    let result = modules.item_usecase().query_items(table_number).await;
    match result {
        Ok(items) => {
            let mut item_responses: Vec<JsonItemResponse> = Vec::new();
            for item in items {
                item_responses.push(JsonItemResponse::new(&item));
            }
            Ok(Json(item_responses))
        }
        Err(e) => match e {
            Error::ItemNotFoundError => Err(StatusCode::NOT_FOUND),
            _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    }
}

pub async fn add_item(
    Path(table_number): Path<u32>,
    Json(req): Json<JsonItemAddingRequest>,
    Extension(modules): Extension<ModulesImpl>,
) -> impl IntoResponse {
    let maybe_item = Item::new(req.name, req.quantity, Local::now().naive_local());
    if maybe_item.is_err() {
        return Err(StatusCode::BAD_REQUEST);
    }
    let result = modules.item_usecase().add_item(table_number, maybe_item.unwrap()).await;
    match result {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_item(
    Path(params): Path<HashMap<String, u32>>,
    Extension(modules): Extension<ModulesImpl>,
) -> impl IntoResponse {
    let result = modules
        .item_usecase()
        .delete_item(
            params.get("table_number").unwrap().clone(),
            params.get("item_id").unwrap().clone(),
        )
        .await;
    match result {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct JsonItemAddingRequest {
    pub name: String,
    pub quantity: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct JsonItemResponse {
    pub id: u32,
    pub name: String,
    pub quantity: u32,
    pub remaining_cooking_minutes: i64,
    pub cooking_started_at: String,
}

impl JsonItemResponse {
    fn new(item: &Item) -> JsonItemResponse {
        JsonItemResponse {
            id: item.id.unwrap(),
            name: item.name.clone(),
            quantity: item.quantity,
            remaining_cooking_minutes: item
                .clone()
                .time_to_finish(Local::now().naive_local())
                .num_minutes(),
            cooking_started_at: item.start_cooking_at.to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use crate::routes;
    use crate::modules::ModulesImpl;
    use axum_test_helper::TestClient;

    async fn test_client() -> TestClient {
        let test_app = routes::router(ModulesImpl::test().await).await;
        TestClient::new(test_app)
    }

    #[tokio::test]
    async fn test_validation_error() {
        let client = test_client().await;
        let empty_name = client.post("/tables/1/item").json(&JsonItemAddingRequest { name: "".to_string(), quantity: 1} ).send().await;
        let empty_quantity = client.post("/tables/1/item").json(&JsonItemAddingRequest { name: "a".to_string(), quantity: 0} ).send().await;
        assert_eq!(empty_name.status(), StatusCode::BAD_REQUEST);
        assert_eq!(empty_quantity.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_post_item() {
        let client = test_client().await;
        let post_item = client.post("/tables/1/item").json(&JsonItemAddingRequest{ name: "Sushi".to_string(), quantity: 10}).send().await;
        assert_eq!(post_item.status(), StatusCode::CREATED);
    }


    #[tokio::test]
    async fn test_query_items() {
        let client = test_client().await;
        client.post("/tables/1/item").json(&JsonItemAddingRequest{ name: "Sushi".to_string(), quantity: 10}).send().await;

        let get_items = client.get("/tables/1/items").send().await;
        assert_eq!(get_items.status(), StatusCode::OK);

        let t = get_items.text().await;
        let response_json: Vec<JsonItemResponse> = serde_json::from_str(&t).unwrap();
        assert_eq!(response_json.len() > 0, true);

        let item_id = response_json[0].id.to_string();
        let get_item = client.get(&format!("/tables/1/items/{}", item_id)).send().await;
        assert_eq!(get_item.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_delete_item() {
        // TODO write test case if axum_test_helper::TestClient supports delete method
    }
}