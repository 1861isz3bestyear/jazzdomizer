mod api;
mod db;
mod display;
mod music_logic;
mod practice_mode;

#[cfg(feature = "cli")]
mod interaction;
#[cfg(feature = "cli")]
mod menu;

use api::{AppState, create_api_router};
use axum::serve;
use db::create_pool;
use tokio::net::TcpListener;

#[cfg(feature = "cli")]
use interaction::{InteractionLayer, console::ConsoleInteraction};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Загружаем .env файл
    dotenvy::dotenv().ok();

    // Создаем пул соединений
    let pool = create_pool().await?;
    println!("Подключение к БД установлено\n");

    #[cfg(feature = "web")]
    {
        let api_pool = pool.clone();
        tokio::spawn(async move {
            if let Err(e) = start_api_server(api_pool).await {
                eprintln!("Api server error: {}", e);
            }
        });
    }

    // Создаем и запускаем меню
    #[cfg(feature = "cli")]
    {
        let menu = menu::StartMenu::new(pool);
        let interaction = ConsoleInteraction;
        menu.run_with_interaction(&interaction).await?;
    }

    //Если только web , ждем завершения

    #[cfg(all(feature = "web", not(feature = "cli")))]
    {
        println!("API server работает");
        tokio::signal::ctrl_c().await?;
        println!("Завершение работы");
    }

    Ok(())
}

#[cfg(feature = "web")]
async fn start_api_server(pool: sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let state = AppState { pool };
    let app = create_api_router(state);

    let host = std::env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_owned());
    let port = std::env::var("APP_PORT").unwrap_or_else(|_| "3000".to_owned());
    let addr = format!("{host}:{port}");
    println!("API server запущен на http://{}", addr);

    let listener = TcpListener::bind(&addr).await?;
    serve(listener, app).await?;

    Ok(())
}
