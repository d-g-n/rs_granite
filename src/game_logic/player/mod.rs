use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{GameState, InGameState};

use self::movement::handle_mouse_movement;

use super::resources::PlayerResource;

mod entity;
mod movement;

pub(crate) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(entity::setup_player.run_if_resource_added::<PlayerResource>())
            .add_system(
                movement::handle_player_movement.run_in_state(GameState::InGame {
                    game_state: InGameState::AwaitingInput,
                }),
            )
            .add_system(entity::handle_player_turn.run_in_state(GameState::InGame {
                game_state: InGameState::PlayerTurn,
            }))
            .add_system(
                handle_mouse_movement
                    .run_if(
                        move |cur_state: Res<CurrentState<GameState>>| match cur_state.0 {
                            GameState::InGame {
                                game_state: InGameState::LoadMap,
                            } => false,
                            GameState::InGame { .. } => true,
                            _ => false,
                        },
                    )
                    .after("renderable_system")
                    .before("render_screen"),
            );
    }
}
