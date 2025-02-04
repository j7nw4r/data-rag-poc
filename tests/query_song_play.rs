use axum::http::StatusCode;
use axum_test::TestServer;
use consts::{POSTGRES_CONNECTION_STRING, SONG_PLAY_ENDPOINT};
use data_rag_poc::{configuration::ConfigurationBuilder, create_router, song_data::{query_song_data::SongPlayRequest, record_song_play::SongPlay}};
use data_rag_poc::song_data::query_song_data::SongPlayResponse;

mod framework;
mod consts;

#[tokio::test]
pub async fn query_song_play_test() {
    framework::init_tracing();

    let config_result = ConfigurationBuilder::default()
        .db_url(POSTGRES_CONNECTION_STRING.to_string())
        .build();
    let Ok(config) = config_result else {
        panic!("could not build the configuration: {:?}", config_result);
    };

    let router_result = create_router(config).await;
    let Ok(router) = router_result else {
        panic!("did not get the Router configuration {:?}", router_result);
    };

    let Ok(server) = TestServer::new(router) else {
        panic!("could not start test server");
    };

    let song_name = random_string::generate(10, random_string::charsets::ALPHANUMERIC);
    record_song_play(&server, &song_name).await;

    let song_play_request = SongPlayRequest {
        name: format!("{}-{}", song_name, "extra-data"),
    };
    let response = server.get(SONG_PLAY_ENDPOINT).json(&song_play_request).await;
    assert_eq!(response.status_code(), StatusCode::OK);
    let resp = response.json::<SongPlayResponse>();

    println!("stored song name: {:?}", song_name);
    println!("queried song name: {:?}", resp.names.iter().next().unwrap());
}

async fn record_song_play(server: &TestServer, song_name: &str) {
    let song_play = SongPlay {
        name: song_name.to_owned(),
    };
    let response = server.post(SONG_PLAY_ENDPOINT).json(&song_play).await;

    assert_eq!(response.status_code(), StatusCode::CREATED);
}