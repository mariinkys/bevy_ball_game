use bevy::prelude::*;

use crate::events::GameOver;
use crate::game::score::resources::*;

pub fn insert_score(mut cmd: Commands) {
    cmd.insert_resource(Score::default());
}

pub fn remove_score(mut cmd: Commands) {
    cmd.remove_resource::<Score>();
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value);
    }
}

pub fn update_high_scores(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_scores: ResMut<HighScores>,
) {
    for event in game_over_event_reader.read() {
        high_scores.scores.push(("Player".to_string(), event.score));
    }
}

pub fn high_scores_updated(high_scores: Res<HighScores>) {
    if high_scores.is_changed() {
        println!("High Scores: {:?}", high_scores);
    }
}
