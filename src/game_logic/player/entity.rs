use std::collections::HashSet;

use bevy::{prelude::*, render::view};
use iyes_loopless::state::NextState;

use crate::{
    game_logic::{
        components::{Blocker, Monster, Player, Position, Renderable, Viewshed},
        map::{game_map::GameMap, pathfinding::astar_next_step},
        resources::PlayerResource,
    },
    screen::structs::ScreenContext,
    GameState, InGameState,
};

pub fn setup_player(mut commands: Commands, player_res: Res<PlayerResource>) {
    commands
        .spawn()
        .insert(Player {})
        .insert(player_res.start_pos.clone())
        .insert(Renderable {
            glyph: '@' as u16,
            fg: Color::YELLOW,
            bg: Color::BLACK,
            layer: 100.0,
        })
        .insert(Blocker {})
        .insert(Viewshed {
            dirty: true,
            distance: 8,
            visible_tiles: HashSet::new(),
        });

    commands
        .spawn()
        .insert(Monster {})
        .insert(player_res.start_pos.clone())
        .insert(Renderable {
            glyph: 'g' as u16,
            fg: Color::RED,
            bg: Color::BLACK,
            layer: 50.0,
        })
        .insert(Blocker {})
        .insert(Viewshed {
            dirty: true,
            distance: 8,
            visible_tiles: HashSet::new(),
        });
}

pub fn handle_player_turn(mut commands: Commands) {
    commands.insert_resource(NextState(GameState::InGame {
        game_state: InGameState::EnemyTurn,
    }));
}
