use axum::serve;
use jazzdomizer_backend::{api, db};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let pool = db::create_pool().await?;
    println!("Подключение к БД установленно");

    let state = api::AppState { pool };
    let app = api::create_api_router(state);

    let host = std::env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_owned());
    let port = std::env::var("APP_PORT").unwrap_or_else(|_| "3000".to_owned());
    let addr = format!("{host}:{port}");
    println!("API server запущен на http://{}", addr);
    println!("Доступные эндпоинты:");
    println!(" GET /api/songs");
    println!(" GET /api/songs/{{id}}");
    println!(" GET /api/songs/{{id}}/transpose/{{steps}}");
    println!(" GET /api/songs/search/{{term}}");
    println!(" GET /api/health");

    let listener = TcpListener::bind(&addr).await?;
    serve(listener, app).await?;

    Ok(())
}
