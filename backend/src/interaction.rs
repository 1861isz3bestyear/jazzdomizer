use crate::display::SongFullInfo;
use async_trait::async_trait;

#[async_trait]
pub trait InteractionLayer: Send + Sync {
    async fn request_song_selection(
        &self,
        songs: Vec<SongFullInfo>,
    ) -> Result<Option<SongFullInfo>, anyhow::Error>;
    async fn show_song_details(&self, song: &SongFullInfo) -> Result<(), anyhow::Error>;
    async fn show_message(&self, message: &str) -> Result<(), anyhow::Error>;
    async fn show_error(&self, error: &str) -> Result<(), anyhow::Error>;
}

#[cfg(feature = "cli")]
pub mod console {
    use super::*;
    use inquire::{Select, Text};

    pub struct ConsoleInteraction;

    #[async_trait]
    impl InteractionLayer for ConsoleInteraction {
        async fn request_song_selection(
            &self,
            songs: Vec<SongFullInfo>,
        ) -> Result<Option<SongFullInfo>, anyhow::Error> {
            if songs.is_empty() {
                self.show_message("Нет песен для выбора").await?;
                return Ok(None);
            }
            let choices: Vec<String> = songs.iter().map(|s| s.description.clone()).collect();
            let choices_with_cancel: Vec<String> = choices
                .into_iter()
                .chain(vec!["Отмена".to_string()])
                .collect();
            let selected = Select::new("Выберите песню:", choices_with_cancel)
                .with_page_size(10)
                .with_help_message("Стрелка вверх/вниз - навигация, Enter - выбор")
                .prompt()?;
            if selected == "Отмена" {
                return Ok(None);
            }
            let index = songs
                .iter()
                .position(|s| {
                    let formated = &s.description;
                    formated.to_string() == selected
                })
                .unwrap();
            Ok(Some(songs[index].clone()))
        }

        async fn show_song_details(&self, song: &SongFullInfo) -> Result<(), anyhow::Error> {
            println!(
                "\n
                Полная информация о песне: \n{}",
                song.description
            );
            println!("\nМузыкальная информация: {}\n", song.music_info);
            Text::new("Нажмите Enter для продолжения").prompt()?;
            Ok(())
        }

        async fn show_message(&self, message: &str) -> Result<(), anyhow::Error> {
            println!("\nInfo! {}", message);
            Ok(())
        }

        async fn show_error(&self, error: &str) -> Result<(), anyhow::Error> {
            eprintln!("\nОшибка: {}", error);
            Ok(())
        }
    }
}
