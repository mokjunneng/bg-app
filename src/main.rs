pub mod prisma;

use std::io;

use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

use bgapp::request::parse_request;
use tokio::net::TcpListener;

fn main() {}

// #[tokio::main]
// async fn main() -> io::Result<()> {
//     // initialize tracing
//     tracing_subscriber::fmt::init();
//
//     let app = Router::new()
//         .route("/", get())
//         .route("/users", post())
//
//     // Listen for incoming TCP connections on localhost port 7878
//     let listener = TcpListener::bind("127.0.0.1:7878").await?;
//     axum::serve(listener, app).await?;
// }
//
