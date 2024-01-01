use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use resources::*;
use systems::*;

use crate::AppState;

use super::SimulationState;

pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0; //Enemy sprite size

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            //Enter State Systems
            .add_systems(OnEnter(AppState::Game), spawn_enemies) // Run when we enter game state
            //Systems
            // .add_systems(
            //     (
            //         enemy_movement,
            //         update_enemy_direction,
            //         confine_enemy_movement,
            //         tick_enemy_spawn_timer,
            //         spawn_enemies_over_time,
            //     )
            //         .in_set(OnUpdate(AppState::Game)) //Run only if AppState is set to Game
            //         .in_set(OnUpdate(SimulationState::Running)), //Run only if SimulationState is set to Running
            // )
            .add_systems(
                Update,
                (
                    enemy_movement,
                    confine_enemy_movement,
                    update_enemy_direction,
                    tick_enemy_spawn_timer,
                    spawn_enemies_over_time,
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            //Exit State Systems
            .add_systems(OnExit(AppState::Game), despawn_enemies); // Run when we exit game state
    }
}
