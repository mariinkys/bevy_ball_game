use bevy::prelude::*;

use super::SimulationState;

pub fn toogle_simulation(
    mut cmd: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if simulation_state.0 == SimulationState::Running {
            cmd.insert_resource(NextState(Some(SimulationState::Paused)));
            println!("Paused Simulation");
        }
        if simulation_state.0 == SimulationState::Paused {
            cmd.insert_resource(NextState(Some(SimulationState::Running)));
            println!("Running Simulation");
        }
    }
}
