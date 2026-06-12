pub mod db;
pub mod display;
pub mod music_logic;
pub mod practice_mode;

#[cfg(feature = "cli")]
pub mod interaction;

#[cfg(feature = "cli")]
pub mod menu;

#[cfg(feature = "web")]
pub mod api;
