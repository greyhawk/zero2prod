#[actix_rt::test]
async fn health_check_works() {
    // 启动应用
    spawn_app();
    // 发送请求
    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:8080/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    // 断言
    assert!(response.status().is_success());
}

fn spawn_app() {
    let server = zero2prod::run().expect("Failed to spawn app.");
    let _ = tokio::spawn(server);
}
