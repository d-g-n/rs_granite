use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{GameState, InGameState};

use super::resources::PlayerResource;

mod entity;

pub(crate) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(entity::setup_player.run_if_resource_added::<PlayerResource>())
            .add_system(
                entity::handle_player_movement.run_in_state(GameState::InGame {
                    game_state: InGameState::AwaitingInput,
                }),
            );
    }
}
