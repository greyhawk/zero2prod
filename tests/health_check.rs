use actix_web::{test, App};
use zero2prod::health_check;

#[actix_rt::test]
async fn health_check_works() {
    // 启动应用
    let app = test::init_service(App::new().service(health_check)).await;

    // 发送请求
    let req = test::TestRequest::get().uri("/health_check").to_request();
    let resp = test::call_service(&app, req).await;

    // 断言
    assert!(resp.status().is_success());
}
