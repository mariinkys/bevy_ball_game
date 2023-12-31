use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}
// Changed this for the #[derive(Default) on the Score struct]
// impl Default for Score {
//     fn default() -> Score {
//         Score { value: 0 }
//     }
// }

#[derive(Resource, Default, Debug)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>,
}
// Changed this for the #[derive(Default) on the HighScores struct]
// impl Default for HighScores {
//     fn default() -> HighScores {
//         HighScores { scores: Vec::new() }
//     }
// }
