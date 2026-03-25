#[tokio::test]
async fn health_check_works() {
    spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:8080/health_check")
        .send()
        .await
        .expect("failed to send request");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = learning_rust::deploy().expect("failed to deploy app");
    let _  = tokio::spawn(server);
}