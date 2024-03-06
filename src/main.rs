pub mod events;
mod game;
mod main_menu;
mod systems;

use bevy::prelude::*;
use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::*;

fn main() {
    App::new()
        //Default Plugins
        .add_plugins(DefaultPlugins)
        //Add State
        .init_state::<AppState>()
        //My Plugins
        .add_plugins(GamePlugin)
        .add_plugins(MainMenuPlugin)
        //Startup Systems
        .add_systems(Startup, spawn_camera)
        //Systems
        .add_systems(Update, transition_to_game_state)
        .add_systems(Update, transition_to_main_menu_state)
        .add_systems(Update, exit_game)
        .add_systems(Update, handle_game_over)
        .run();
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
