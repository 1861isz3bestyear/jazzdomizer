use axum::{
    body::{to_bytes, Body},
    http::{header, Method, Request, StatusCode},
};
use jazzdomizer_backend::{
    api::{create_api_router, ApiResponse, AppState},
    db::PgPoolOptions,
};
use tower::util::ServiceExt;

fn app() -> axum::Router {
    let pool = PgPoolOptions::new()
        .connect_lazy("postgres://jazzdomizer:jazzdomizer@localhost/jazzdomizer_test")
        .expect("lazy Postgres pool should be created without connecting");

    create_api_router(AppState { pool })
}

async fn response_json<T>(response: axum::response::Response) -> T
where
    T: serde::de::DeserializeOwned,
{
    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("response body should be readable");

    serde_json::from_slice(&body).expect("response body should be valid JSON")
}

#[tokio::test]
async fn health_endpoint_returns_success_envelope_without_database_access() {
    let response = app()
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/api/health")
                .body(Body::empty())
                .expect("request should be valid"),
        )
        .await
        .expect("router should produce a response");

    assert_eq!(response.status(), StatusCode::OK);

    let body: ApiResponse<()> = response_json(response).await;
    assert!(body.success);
    assert_eq!(body.data, None);
    assert_eq!(body.error, None);
}

#[tokio::test]
async fn health_endpoint_includes_cors_headers_for_browser_clients() {
    let response = app()
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/api/health")
                .header(header::ORIGIN, "http://localhost:5173")
                .body(Body::empty())
                .expect("request should be valid"),
        )
        .await
        .expect("router should produce a response");

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get(header::ACCESS_CONTROL_ALLOW_ORIGIN),
        Some(&header::HeaderValue::from_static("*"))
    );
}
