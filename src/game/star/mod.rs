use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use resources::*;
use systems::*;

use super::SimulationState;
use crate::AppState;

pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0; //Star sprite size

pub struct StarPlugin;
impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            //On Enter State
            .add_system(spawn_stars.in_schedule(OnEnter(AppState::Game))) // Run when we enter game state
            //Systems
            .add_systems(
                (tick_star_spawn_timer, spawn_stars_over_time)
                    .in_set(OnUpdate(AppState::Game)) //Run only if AppState is set to Game
                    .in_set(OnUpdate(SimulationState::Running)), //Run only if SimulationState is set to Running
            )
            //Exit State Systems
            .add_system(despawn_stars.in_schedule(OnExit(AppState::Game))); // Run when we exit game state
    }
}
