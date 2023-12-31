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
            .add_system(spawn_enemies.in_schedule(OnEnter(AppState::Game))) // Run when we enter game state
            //Systems
            // .add_system(enemy_movement)
            // .add_system(confine_enemy_movement)
            // .add_system(update_enemy_direction)
            // .add_system(tick_enemy_spawn_timer)
            // .add_system(spawn_enemies_over_time)
            .add_systems(
                (
                    enemy_movement,
                    confine_enemy_movement,
                    update_enemy_direction,
                    tick_enemy_spawn_timer,
                    spawn_enemies_over_time,
                )
                    .in_set(OnUpdate(AppState::Game)) //Run only if AppState is set to Game
                    .in_set(OnUpdate(SimulationState::Running)), //Run only if SimulationState is set to Running
            )
            //Exit State Systems
            .add_system(despawn_enemies.in_schedule(OnExit(AppState::Game))); // Run when we exit game state
    }
}
