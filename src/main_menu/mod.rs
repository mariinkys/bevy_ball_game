mod components;
mod styles;
mod systems;

use bevy::prelude::*;

use crate::AppState;

use self::systems::{interactions::*, layout::*};

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            //On Enter State
            .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            //Systems
            .add_systems(
                Update,
                (interact_with_play_button, interact_with_quit_button)
                    .run_if(in_state(AppState::MainMenu)),
            )
            //On Exit State
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
    }
}
