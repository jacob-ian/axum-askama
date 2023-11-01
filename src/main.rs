use askama_axum::assets;
use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_world))
        .nest("/assets", assets::router());

    println!("Listening on 0.0.0.0:4000");
    axum::Server::bind(&"0.0.0.0:4000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello_world() -> Html<&'static str> {
    return Html("<h1>Hello world</h1>");
}
