use bevy::prelude::*;

use super::SimulationState;

pub fn pause_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    simulation_state_next_state.set(SimulationState::Paused);
}

pub fn resume_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    simulation_state_next_state.set(SimulationState::Running);
}

pub fn toogle_simulation(
    mut cmd: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if simulation_state.get() == &SimulationState::Running {
            cmd.insert_resource(NextState(Some(SimulationState::Paused)));
            println!("Paused Simulation");
        }
        if simulation_state.get() == &SimulationState::Paused {
            cmd.insert_resource(NextState(Some(SimulationState::Running)));
            println!("Running Simulation");
        }
    }
}
