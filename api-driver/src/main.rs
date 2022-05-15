use api_driver::modules::ModulesImpl;

#[tokio::main]
async fn main() {
    let app = api_driver::routes::router(ModulesImpl::new().await).await;
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
