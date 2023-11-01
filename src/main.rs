use askama::Template;
use axum::{routing::get, Router};
use try_askama_axum::assets;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest("/assets", assets::router())
        .route("/", get(home));

    println!("Listening on 0.0.0.0:4000");
    axum::Server::bind(&"0.0.0.0:4000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate<'a> {
    name: &'a str,
}

async fn home() -> HomeTemplate<'static> {
    let home = HomeTemplate { name: "Jacob" };
    return home;
}
