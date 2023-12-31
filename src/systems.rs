use crate::{events::*, game::SimulationState, AppState};
use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};

pub fn spawn_camera(mut cmd: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    cmd.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn transition_to_game_state(
    mut cmd: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::G) && app_state.0 != AppState::Game {
        cmd.insert_resource(NextState(Some(AppState::Game)));
        println!("Entered Game AppState")
    }
}

pub fn transition_to_main_menu_state(
    mut cmd: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::M) && app_state.0 != AppState::MainMenu {
        cmd.insert_resource(NextState(Some(AppState::MainMenu)));
        cmd.insert_resource(NextState(Some(SimulationState::Paused)));
        println!("Entered Main Menu AppState")
    }
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn handle_game_over(mut cmd: Commands, mut game_over_event_reader: EventReader<GameOver>) {
    for event in game_over_event_reader.iter() {
        println!("Your final score is: {}", event.score);
        cmd.insert_resource(NextState(Some(AppState::GameOver)));
        cmd.insert_resource(NextState(Some(SimulationState::Paused)));
    }
}
