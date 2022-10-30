use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{GameState, InGameState};

mod entity;

pub(crate) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_exit_system(GameState::Loading, entity::setup_player)
            .add_system(
                entity::handle_player_movement.run_in_state(GameState::InGame {
                    game_state: InGameState::AwaitingInput,
                }),
            );
    }
}
