#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = std::net::TcpListener::bind("127.0.0.1:8000")?;
    zero2prod::run(listener)?.await
}
