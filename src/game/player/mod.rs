use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

use crate::AppState;

use super::SimulationState;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinementSystemSet;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(MovementSystemSet.before(ConfinementSystemSet))
            //On Enter State
            .add_system(spawn_player.in_schedule(OnEnter(AppState::Game))) // Run when we enter game state
            //Systems
            .add_system(
                player_movement
                    .in_set(MovementSystemSet)
                    .run_if(in_state(AppState::Game)) //Run only if AppState is set to Game
                    .run_if(in_state(SimulationState::Running)), //Run only if SimulationState is set to Running
            )
            .add_system(
                confine_player_movement
                    .in_set(ConfinementSystemSet)
                    .run_if(in_state(AppState::Game)) //Run only if AppState is set to Game
                    .run_if(in_state(SimulationState::Running)), //Run only if SimulationState is set to Running
            )
            .add_systems(
                (enemy_hit_player, player_hit_star)
                    .in_set(OnUpdate(AppState::Game)) //Run only if AppState is set to Game
                    .in_set(OnUpdate(SimulationState::Running)), //Run only if SimulationState is set to Running
            )
            //Exit State Systems
            .add_system(despawn_player.in_schedule(OnExit(AppState::Game))); // Run when we exit game state
    }
}
