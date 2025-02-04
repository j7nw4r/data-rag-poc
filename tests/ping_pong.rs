use axum::http::StatusCode;
use axum_test::TestServer;
use data_rag_poc::create_router;
use data_rag_poc::configuration::ConfigurationBuilder;

#[tokio::test]
async fn basic_ping_pong() {
    let config_result = ConfigurationBuilder::default().build();
    let Ok(config) = config_result else {
        panic!("could not build the configuration: {:?}", config_result);
    };

    let Ok(router) = create_router(config).await else {
        panic!("did not get the Router configuration");
    };

    let Ok(server) = TestServer::new(router) else {
        panic!("could not start test server");
    };

    let response = server.get("/ping").await;

    assert_eq!(response.status_code(), StatusCode::OK);
}
