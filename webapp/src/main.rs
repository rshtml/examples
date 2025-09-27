use axum::Router;
use axum::response::Html;
use axum::routing::get;
use chrono::{DateTime, Datelike, Utc};
use rshtml::{RsHtml,traits::RsHtml,functions::*};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(index))
        .nest_service("/css", ServeDir::new("views/css"))
        .nest_service("/img", ServeDir::new("views/img"))
        .nest_service("/js", ServeDir::new("views/js"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("✅ Server started at http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Html<String> {
    let mut index_page = IndexPage {
        title: "RsHtml Web Page".to_string(),
        email: "contact@rshtml.com".to_string(),
        footer: true,
        navbar_sections: vec![
            ("#tm-section-1", "Home"),
            ("#tm-section-2", "Services"),
            ("#tm-section-3", "About"),
            ("#tm-section-4", "Contact"),
        ],
        home_time: Utc::now(),
    };

    let index = index_page.render().unwrap();

    Html(index)
}

#[derive(RsHtml)]
//#[rshtml(path = "index.rs.html")]
struct IndexPage {
    pub title: String,
    pub email: String,
    pub footer: bool,
    pub navbar_sections: Vec<(&'static str, &'static str)>,
    pub home_time: DateTime<Utc>,
}
