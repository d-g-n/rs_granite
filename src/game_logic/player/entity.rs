use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    game_logic::{
        components::{Blocker, Player, Position, Renderable},
        map::{pathfinding::astar_next_step, GameMap},
    },
    screen::{ScreenContext, ScreenTile},
};

pub fn setup_player(mut commands: Commands) {
    commands
        .spawn()
        .insert(Player {})
        .insert(Position { x: 0, y: 0 })
        .insert(Renderable {
            glyph: '@' as u16,
            fg: Color::YELLOW,
            bg: Color::BLACK,
            layer: 100.0,
        })
        .insert(Blocker {});
}

#[derive(Default)]
pub struct HeldCounter {
    counter_ms: u128,
    passed_initial_threshold: bool,
}

pub fn handle_player_movement(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    mut ctx: ResMut<ScreenContext>,
    time: Res<Time>,
    map: Res<GameMap>,
    mut held_counter: Local<HeldCounter>,
    mut player_position_query: Query<(Entity, &mut Position), With<Player>>,
    blocker_position_query: Query<(Entity, &Position), (With<Blocker>, Without<Player>)>,
) {
    let (direction_x, direction_y) =
        if keyboard.pressed(KeyCode::W) || keyboard.pressed(KeyCode::Up) {
            (0, 1)
        } else if keyboard.pressed(KeyCode::A) || keyboard.pressed(KeyCode::Left) {
            (-1, 0)
        } else if keyboard.pressed(KeyCode::S) || keyboard.pressed(KeyCode::Down) {
            (0, -1)
        } else if keyboard.pressed(KeyCode::D) || keyboard.pressed(KeyCode::Right) {
            (1, 0)
        } else {
            if held_counter.counter_ms != 0 {
                *held_counter = HeldCounter::default();
            }

            (0, 0)
        };

    let (_entity, mut player_pos) = player_position_query.single_mut();

    let new_x = player_pos.x + direction_x;
    let new_y = player_pos.y + direction_y;

    for (entity, position) in blocker_position_query.iter() {
        if position.x == new_x && position.y == new_y {
            return;
        }
    }

    if new_x >= 0
        && new_y >= 0
        && new_x < ctx.width as i32
        && new_y < ctx.height as i32
        && held_counter.counter_ms == 0
        && (direction_x != 0 || direction_y != 0)
    {
        player_pos.x = new_x;
        player_pos.y = new_y;

        let astar = astar_next_step(&map, player_pos.clone(), Position { x: 1, y: 0 });

        if let Some((res_vec, _no)) = astar {
            for pos in res_vec.iter() {
                let mut tile = ctx.get_tile(pos.x as usize, pos.y as usize);

                tile.bg_color = Color::RED;
            }
        }
    }

    if !(direction_x == 0 && direction_y == 0) {
        held_counter.counter_ms += time.delta().as_millis();
    }

    if held_counter.passed_initial_threshold {
        if held_counter.counter_ms >= 100 {
            held_counter.counter_ms = 0;
        }
    } else {
        if held_counter.counter_ms >= 300 {
            held_counter.passed_initial_threshold = true;
        }
    }
}
