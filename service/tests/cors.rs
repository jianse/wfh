use axum::{
    Json, Router,
    http::{HeaderValue, Method},
    response::{Html, IntoResponse},
    routing::get,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[tokio::test]
async fn main() {
    let frontend = async {
        let app = Router::new().route("/", get(html));
        serve(app, 3050).await;
    };

    let backend = async {
        let app = Router::new().route("/json", get(json)).layer(
            // see https://docs.rs/tower-http/latest/tower_http/cors/index.html
            // for more details
            //
            // pay attention that for some request types like posting content-type: application/json
            // it is required to add ".allow_headers([http::header::CONTENT_TYPE])"
            // or see this issue https://github.com/tokio-rs/axum/issues/849
            CorsLayer::new()
                .allow_origin("http://localhost:3050".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET]),
        );
        serve(app, 4050).await;
    };

    tokio::join!(frontend, backend);
}

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn html() -> impl IntoResponse {
    Html(
        r#"
        <script>
            fetch('http://localhost:4050/json')
              .then(response => response.json())
              .then(data => console.log(data));
        </script>
        "#,
    )
}

async fn json() -> impl IntoResponse {
    Json(vec!["one", "two", "three"])
}
