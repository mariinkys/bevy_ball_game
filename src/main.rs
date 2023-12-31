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
        .add_state::<AppState>()
        //My Plugins
        .add_plugin(GamePlugin)
        .add_plugin(MainMenuPlugin)
        //Startup Systems
        .add_startup_system(spawn_camera)
        //Systems
        .add_system(transition_to_game_state)
        .add_system(transition_to_main_menu_state)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .run();
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
