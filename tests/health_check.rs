use std::net::TcpListener;

#[actix_rt::test]
async fn health_check_works() {
    // 启动应用
    let address = spawn_app();
    // 发送请求
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request.");

    // 断言
    assert!(response.status().is_success());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to spawn app.");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
