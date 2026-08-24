use axum::Router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let router = Router::new().route("/", );
    axum::serve(listener, router).await.unwrap();
}
