mod framework;
mod consts;

use axum::http::StatusCode;
use axum_test::TestServer;
use data_rag_poc::{configuration::ConfigurationBuilder, create_router, song_data::record_song_play::SongPlay};
use crate::consts::SONG_PLAY_ENDPOINT;

#[tokio::test]
pub async fn record_song_play_test() {
    framework::init_tracing();
    
    let config_result = ConfigurationBuilder::default()
        .db_url(consts::POSTGRES_CONNECTION_STRING.to_string())
        .build();
    let Ok(_config) = config_result else {
        panic!("could not build the configuration: {:?}", config_result);
    };

    let router_result = create_router(_config).await;
    let Ok(router) = router_result else {
        panic!("did not get the Router configuration {:?}", router_result);
    };

    let Ok(server) = TestServer::new(router) else {
        panic!("could not start test server");
    };

    let song_play = SongPlay {
        name: "Song 1".to_string(),
    };
    
    let response = server.post(SONG_PLAY_ENDPOINT).json(&song_play).await;

    assert_eq!(response.status_code(), StatusCode::CREATED);

    println!("response text: {:?}", response.text());
}
