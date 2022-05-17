use api_driver::controllers::item::{
    JsonItemAddingRequest, JsonItemAddlingResponse, JsonItemResponse,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let post_item_requests = vec![
        JsonItemAddingRequest {
            name: "Sukiyaki".to_string(),
            quantity: 2,
        },
        JsonItemAddingRequest {
            name: "Okonomiyaki".to_string(),
            quantity: 5,
        },
    ];

    let mut item_ids: Vec<u64> = Vec::new();
    for request in post_item_requests {
        let response = client
            .post(add_item_endpoint("1"))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        let response_json: JsonItemAddlingResponse =
            serde_json::from_str(&response.text().await.unwrap()).unwrap();
        item_ids.push(response_json.item_id);
        println!("{:#?}", response_json);
    }

    let query_items_response = client.get(query_items_endpoint("1")).send().await?;
    let query_items_response_json: Vec<JsonItemResponse> =
        serde_json::from_str(&query_items_response.text().await.unwrap()).unwrap();
    println!("{:#?}", query_items_response_json);

    let query_item_response = client
        .get(query_item_endpoint("1", &item_ids[0].to_string()))
        .send()
        .await?;
    let query_item_response_json: JsonItemResponse =
        serde_json::from_str(&query_item_response.text().await.unwrap()).unwrap();
    println!("{:#?}", query_item_response_json);

    let delete_item_response = client
        .delete(delete_item_endpoint("1", &item_ids[0].to_string()))
        .send()
        .await?;
    println!("{:#?}", delete_item_response);

    Ok(())
}

fn api_url() -> &'static str {
    "http://localhost:3000"
}

fn add_item_endpoint(table_number_str: &str) -> String {
    format!("{}/tables/{}/item", api_url(), table_number_str)
}

fn query_item_endpoint(table_number_str: &str, item_id_str: &str) -> String {
    format!(
        "{}/tables/{}/items/{}",
        api_url(),
        table_number_str,
        item_id_str
    )
}

fn query_items_endpoint(table_number_str: &str) -> String {
    format!("{}/tables/{}/items", api_url(), table_number_str)
}

fn delete_item_endpoint(table_number_str: &str, item_id_str: &str) -> String {
    format!(
        "{}/tables/{}/items/{}",
        api_url(),
        table_number_str,
        item_id_str
    )
}
