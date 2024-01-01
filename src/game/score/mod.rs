use bevy::prelude::*;

pub mod resources;
mod systems;

use resources::*;
use systems::*;

use crate::AppState;

pub struct ScorePlugin;
impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HighScores>()
            //On Enter State
            .add_systems(OnEnter(AppState::Game), insert_score)
            //Systems
            .add_systems(Update, update_score.run_if(in_state(AppState::Game)))
            .add_systems(Update, update_high_scores)
            .add_systems(Update, high_scores_updated)
            //On Exit State
            .add_systems(OnExit(AppState::Game), remove_score);
    }
}
