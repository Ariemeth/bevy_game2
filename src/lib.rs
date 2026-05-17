pub mod game;
pub mod ui;

pub use crate::game::game::GamePlugin;
pub use crate::ui::primary_game_screen::PrimaryGameScreenPlugin as PrimaryGameUIPlugin;
pub use crate::ui::start_menu::StartMenuPlugin;
pub use crate::game::state::GameState;
