use zero2prod::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = "127.0.0.1:8080";
    let listener = std::net::TcpListener::bind(address)?;
    run(listener)?.await
}
