use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{GameState, InGameState};

mod ai;

pub(crate) struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(ai::handle_monster_movement.run_in_state(GameState::InGame {
            game_state: InGameState::EnemyTurn,
        }));
    }
}
