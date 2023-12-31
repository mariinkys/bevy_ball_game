use bevy::prelude::*;

pub const STAR_SPAWN_TIME: f32 = 1.0; //Time between stars spawning (in seconds)
pub const ENEMY_SPAWN_TIME: f32 = 2.5; //Time between enemies spawning (in seconds)

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

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}
impl Default for StarSpawnTimer {
    fn default() -> StarSpawnTimer {
        StarSpawnTimer {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}
impl Default for EnemySpawnTimer {
    fn default() -> EnemySpawnTimer {
        EnemySpawnTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}
