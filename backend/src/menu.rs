use crate::display::SongFullInfo;
use crate::display::SongPreviewInfo;
use crate::practice_mode::{GlobalSongList, LocalSongList};
use sqlx::PgPool;
use std::io::{self, Write};

#[cfg(feature = "cli")]
use crate::interaction::InteractionLayer;

pub struct StartMenu {
    pool: PgPool,
}

impl StartMenu {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("\nJAZZDOMIZER\n");

        loop {
            self.show_menu().await?;
            let choice = self.read_input().await?;
            if !self.start_menu_choice(&choice).await? {
                break;
            }
        }
        Ok(())
    }

    async fn show_menu(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("\nГЛАВНОЕ МЕНЮ\n");
        println!("Нажмите 1 для списка всех песен");
        println!("Нажмите 2 для выбора одной песни");
        println!("Нажмите 3 для списка песен в режиме тренировки");
        println!("Нажмите 0 для выхода");
        print!("Выберите действие: ");

        io::stdout().flush()?;
        Ok(())
    }

    async fn read_input(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_string())
    }

    async fn start_menu_choice(&self, choice: &str) -> Result<bool, Box<dyn std::error::Error>> {
        match choice {
            "1" => self.handle_song_list().await?,
            "2" => self.handle_song().await?,
            "3" => self.handle_practice_song_list().await?,
            "0" => {
                println!("Завершение работы");
                return Ok(false);
            }
            _ => {
                println!("Неверный ввод")
            }
        }
        Ok(true)
    }

    async fn handle_song_list(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Загрузка списка песен");
        let songs: Vec<SongPreviewInfo> = SongPreviewInfo::get_all(&self.pool).await?;
        println!("Найдено {} песен", songs.len());
        if !songs.is_empty() {
            println!("\nСписок песен:");
            for song in songs {
                let display_song = SongPreviewInfo::show_default(&self.pool, song.id).await?;
                println!("-{}", display_song);
            }
        } else {
            println!("Таблица песен пуста");
        }
        Ok(())
    }

    async fn handle_song(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Впишите название песни");
        let search_term = self.read_input().await?;
        let songs = SongFullInfo::get_songs_by_name(&self.pool, &search_term).await?;
        println!("С именем {} найденно {} песен", &search_term, songs.len());
        for song in songs {
            let display_song = SongFullInfo::show_default(&self.pool, song.id).await?;
            println!("\n{}", display_song);
        }
        Ok(())
    }

    async fn handle_practice_song_list(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n Режим тренировки");
        let songs: Vec<SongPreviewInfo> = SongPreviewInfo::get_all(&self.pool).await?;
        println!("Найдено {} песен", songs.len());
        if songs.is_empty() {
            println!("Таблица песен пуста");
            return Ok(());
        }
        println!("\nСписок песен:");
        for song in songs {
            let display_song = SongPreviewInfo::alter_by_db(&song, &self.pool).await?;
            println!("-{}", display_song);
        }
        Ok(())
    }

    //CLI FEATURE CONFIG
    #[cfg(feature = "cli")]
    pub async fn run_with_interaction(
        &self,
        interaction: &dyn InteractionLayer,
    ) -> Result<(), Box<dyn std::error::Error>> {
        interaction
            .show_message("Система управления базой данных")
            .await?;

        loop {
            self.show_menu().await?;
            let choice = self.read_input().await?;
            if !self
                .start_menu_choice_with_interaction(&choice, interaction)
                .await?
            {
                break;
            }
        }
        Ok(())
    }

    #[cfg(feature = "cli")]
    async fn start_menu_choice_with_interaction(
        &self,
        choice: &str,
        interaction: &dyn InteractionLayer,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        match choice {
            "1" => self.handle_song_list_with_interaction(interaction).await?,
            "2" => self.handle_song_with_interaction(interaction).await?,
            "3" => {
                self.handle_practice_song_list_with_interaction(interaction)
                    .await?
            }
            "0" => {
                return Ok(false);
            }
            _ => {
                interaction.show_error("Неверный ввод").await?;
            }
        }
        Ok(true)
    }

    #[cfg(feature = "cli")]
    async fn handle_song_list_with_interaction(
        &self,
        interaction: &dyn InteractionLayer,
    ) -> Result<(), Box<dyn std::error::Error>> {
        interaction.show_message("Загрузка песен").await?;
        let songs: Vec<SongPreviewInfo> = SongPreviewInfo::get_all(&self.pool).await?;
        let message = format!("Найдено {} песен", songs.len());
        interaction.show_message(&message).await?;
        if !songs.is_empty() {
            interaction.show_message("Список песен:").await?;
            for song in songs {
                let display_song = SongPreviewInfo::show_default(&self.pool, song.id);
                interaction
                    .show_message(&format!("-{}", display_song.await?))
                    .await?;
            }
        } else {
            interaction.show_message("Таблица песен пуста").await?;
        }
        Ok(())
    }

    #[cfg(feature = "cli")]
    async fn handle_song_with_interaction(
        &self,
        interaction: &dyn InteractionLayer,
    ) -> Result<(), Box<dyn std::error::Error>> {
        interaction.show_message("Впишите название песни").await?;
        let search_term = self.read_input().await?;
        let songs = SongFullInfo::get_songs_by_name(&self.pool, &search_term).await?;
        if songs.is_empty() {
            println!("Песен с названием {} не найденно", search_term);
            return Ok(());
        }
        //интерактивный выбор
        let selected_song = if songs.len() == 1 {
            interaction.show_message("Найденна одна песня:").await?;
            songs.into_iter().next().unwrap()
        } else {
            interaction
                .show_message(&format!("Найденно песен {}", songs.len()))
                .await?;
            let choices: Vec<(String, usize)> = songs
                .iter()
                .enumerate()
                .map(|(idx, s)| {
                    let display = s.description.clone();
                    (display, idx)
                })
                .collect();
            let choice_strings: Vec<String> =
                choices.iter().map(|(display, _)| display.clone()).collect();
            let selected = inquire::Select::new("Выберите одну песню", choice_strings)
                .with_page_size(10)
                .with_help_message("↑/↓ - навигация, Enter - выбор")
                .prompt()?;
            let index = choices
                .iter()
                .find(|(display, _)| display == &selected)
                .map(|(_, idx)| *idx);
            match index {
                Some(idx) => songs.into_iter().nth(idx).unwrap(),
                None => {
                    interaction
                        .show_error("Не удалось определить выбранную песню")
                        .await?;
                    return Ok(());
                }
            }
        };
        //показываем детальную информацию
        interaction.show_song_details(&selected_song).await?;
        Ok(())
    }

    #[cfg(feature = "cli")]
    async fn handle_practice_song_list_with_interaction(
        &self,
        interaction: &dyn InteractionLayer,
    ) -> Result<(), Box<dyn std::error::Error>> {
        interaction.show_message("Режим тренировки").await?;
        let has_practice_records = LocalSongList::check_practice_records(&self.pool).await?;
        let mut playlist = if has_practice_records {
            interaction
                .show_message("Продолжение существующей сессии...")
                .await?;
            LocalSongList::continue_session(&self.pool).await?
        } else {
            interaction
                .show_message("Создание новой сессии тренировки...")
                .await?;
            interaction
                .show_message("Инициализация глобального плейлиста...")
                .await?;
            GlobalSongList::new(4, &self.pool).await?;
            interaction.show_message("Начало новой сессии...").await?;
            LocalSongList::continue_session(&self.pool).await?
        };
        // Запуск режима тренировки
        self.run_practice_session(&mut playlist, interaction)
            .await?;
        Ok(())
    }

    #[cfg(feature = "cli")]
    async fn run_practice_session(
        &self,
        playlist: &mut LocalSongList,
        interaction: &dyn InteractionLayer,
    ) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            // Подготавливаем список песен для выбора
            let song_choices: Vec<String> = playlist
                .preview_songs
                .iter()
                .enumerate()
                .map(|(idx, song)| {
                    let harmony = song.harmony_key.as_str();
                    format!("{}. {} [{}]", idx + 1, song.song_name, harmony)
                })
                .collect();
            // Добавляем дополнительные опции в меню
            let mut menu_items = song_choices;
            menu_items.push("Перейти к следующему списку".to_string());
            menu_items.push("Начать глобальный список заново".to_string());
            menu_items.push("Выйти в предыдущее меню".to_string());
            interaction
                .show_message("\n=== РЕЖИМ ТРЕНИРОВКИ ===")
                .await?;
            interaction
                .show_message(&format!(
                    "Всего песен в плейлисте: {}",
                    playlist.preview_songs.len()
                ))
                .await?;
            interaction
                .show_message("\nВыберите песню для просмотра или действие:\n")
                .await?;
            let selected = inquire::Select::new("Ваш выбор:", menu_items)
                .with_page_size(15)
                .with_help_message("↑/↓ - навигация, Enter - выбор")
                .prompt()?;
            // Обработка выбора
            if selected.starts_with("Перейти к следующему") {
                // Переход к следующему списку
                interaction
                    .show_message("\nПодготовка следующей сессии...")
                    .await?;
                match GlobalSongList::next_local_playlist(&self.pool).await {
                    Ok(_) => {
                        // Обновляем текущий плейлист
                        match LocalSongList::continue_session(&self.pool).await {
                            Ok(new_playlist) => {
                                *playlist = new_playlist;
                                interaction
                                    .show_message(&format!(
                                        "✓ Загружен новый плейлист с {} песнями",
                                        playlist.preview_songs.len()
                                    ))
                                    .await?;
                            }
                            Err(e) => {
                                interaction
                                    .show_error(&format!("Ошибка загрузки нового плейлиста: {}", e))
                                    .await?;
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        interaction
                            .show_error(&format!("Ошибка перехода к следующему списку: {}", e))
                            .await?;
                        break;
                    }
                }
            } else if selected.starts_with("Начать глобальный") {
                // Начать GlobalSongList заново
                interaction
                    .show_message("\nПерезапуск GlobalSongList...")
                    .await?;
                match GlobalSongList::new(4, &self.pool).await {
                    Ok(_) => match LocalSongList::continue_session(&self.pool).await {
                        Ok(new_playlist) => {
                            *playlist = new_playlist;
                            interaction
                                .show_message("✓ GlobalSongList успешно перезапущен!")
                                .await?;
                            interaction
                                .show_message(&format!(
                                    "Новый плейлист содержит {} песен",
                                    playlist.preview_songs.len()
                                ))
                                .await?;
                        }
                        Err(e) => {
                            interaction
                                .show_error(&format!("Ошибка загрузки плейлиста: {}", e))
                                .await?;
                            break;
                        }
                    },
                    Err(e) => {
                        interaction
                            .show_error(&format!("Ошибка перезапуска: {}", e))
                            .await?;
                        break;
                    }
                }
            } else if selected.starts_with("Выйти в") {
                // Выход в предыдущее меню
                interaction
                    .show_message("Возврат в главное меню...")
                    .await?;
                break;
            } else {
                // Выбрана песня - извлекаем индекс
                if let Some(idx) = selected.split('.').next() {
                    if let Ok(song_index) = idx.parse::<usize>() {
                        let selected_song = &playlist.preview_songs[song_index - 1];
                        // Показываем детальную информацию о песне
                        interaction
                            .show_message(&format!("\n=== {} ===", selected_song.song_name))
                            .await?;
                        // Получаем и показываем полную информацию о песне
                        match SongFullInfo::alter_by_db(&self.pool, selected_song.song_id).await {
                            Ok(song_info) => {
                                interaction.show_song_details(&song_info).await?;
                                // Дополнительно показываем гармонию отдельно, если она есть
                                if let harmony = &selected_song.harmony_key {
                                    interaction.show_message(&format!(
                                    "\n🎵 Гармония: {} - используйте эту тональность для практики",
                                    harmony
                                )).await?;
                                } else {
                                    interaction
                                        .show_message("\n🎵 Гармония не указана для этой песни")
                                        .await?;
                                }
                            }
                            Err(e) => {
                                interaction
                                    .show_error(&format!("Ошибка загрузки деталей песни: {}", e))
                                    .await?;
                            }
                        }
                        // Пауза перед возвратом к списку
                        interaction
                            .show_message("\nНажмите Enter, чтобы вернуться к списку...")
                            .await?;
                        self.read_input().await?;
                    }
                }
            }
        }

        Ok(())
    }
}
