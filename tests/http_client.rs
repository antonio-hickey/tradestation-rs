use mockito::Server;
use reqwest::header::{HeaderMap, HeaderValue};
use tradestation::{ClientBuilder, ClientEnvironment, Token};

#[tokio::test]
async fn uses_configured_http_client_for_api_requests() {
    let mut headers = HeaderMap::new();
    headers.insert("x-transport", HeaderValue::from_static("custom"));

    let http_client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/transport-test")
        .match_header("x-transport", "custom")
        .with_status(200)
        .with_body("ok")
        .create_async()
        .await;

    let client = ClientBuilder::new()
        .http_client(http_client)
        .environment(ClientEnvironment::Mock(server.url()))
        .with_token(Token::dummy())
        .build()
        .await
        .unwrap();

    let response = client.get("transport-test").await.unwrap();

    assert_eq!(response.status(), reqwest::StatusCode::OK);
    mock.assert_async().await;
}
