pub mod game;
pub mod ui;

pub use crate::game::game::GamePlugin;
pub use crate::ui::primary_game::PrimaryGamePlugin as PrimaryGameUIPlugin;
pub use crate::ui::menu::MenuPlugin;
pub use crate::game::state::GameState;
