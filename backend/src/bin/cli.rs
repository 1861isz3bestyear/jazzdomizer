use jazzdomizer_backend::{
    db,
    interaction::{InteractionLayer, console::ConsoleInteraction},
    menu::StartMenu,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let pool = db::create_pool().await?;
    println!("Соединение с БД установлено\n");

    let menu = StartMenu::new(pool);
    let interaction = ConsoleInteraction;

    menu.run_with_interaction(&interaction).await?;

    Ok(())
}
